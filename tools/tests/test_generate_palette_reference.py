"""Exercise distribution/import binding with real filesystem packages."""

import importlib
from importlib import util
from pathlib import Path
import sys
import tempfile
import unittest


SPEC = util.spec_from_file_location(
    "palette_generator", Path(__file__).resolve().parents[1] / "generate_palette_reference.py"
)
GENERATOR = util.module_from_spec(SPEC)
SPEC.loader.exec_module(GENERATOR)
PACKAGE = "iris_palette_reference_oracle_test"


class DistributionBinding(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="iris-reference-import-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.installed = self.root / "installed"
        self.foreign = self.root / "foreign"
        for directory in (self.installed, self.foreign):
            package = directory / PACKAGE
            package.mkdir(parents=True)
            (package / "__init__.py").write_text(f"IDENTITY = {directory.name!r}\n")
        info = self.installed / f"{PACKAGE}-1.0.dist-info"
        info.mkdir()
        (info / "METADATA").write_text(f"Metadata-Version: 2.1\nName: {PACKAGE}\nVersion: 1.0\n")
        self.original_path = sys.path[:]
        self.addCleanup(self.restore_imports)
        sys.path.insert(0, str(self.installed))
        importlib.invalidate_caches()

    def restore_imports(self):
        sys.path[:] = self.original_path
        sys.modules.pop(PACKAGE, None)
        importlib.invalidate_caches()

    def test_matching_distribution_and_import_are_used(self):
        distribution, directory = GENERATOR.bound_distribution(PACKAGE, "1.0")
        module = GENERATOR.import_verified(PACKAGE, directory)
        self.assertEqual(distribution.version, "1.0")
        self.assertEqual(directory, self.installed / PACKAGE)
        self.assertEqual(Path(module.__file__), directory / "__init__.py")
        self.assertEqual(module.IDENTITY, "installed")

    def test_foreign_package_is_rejected_before_import(self):
        sys.path.insert(0, str(self.foreign))
        importlib.invalidate_caches()
        with self.assertRaisesRegex(ValueError, "import resolution differs"):
            GENERATOR.bound_distribution(PACKAGE, "1.0")
        self.assertNotIn(PACKAGE, sys.modules)

    def test_import_redirect_after_binding_is_rejected(self):
        _, directory = GENERATOR.bound_distribution(PACKAGE, "1.0")
        sys.path.insert(0, str(self.foreign))
        importlib.invalidate_caches()
        with self.assertRaisesRegex(ValueError, "imported package differs"):
            GENERATOR.import_verified(PACKAGE, directory)
        self.assertEqual(sys.modules[PACKAGE].IDENTITY, "foreign")


if __name__ == "__main__":
    unittest.main()
