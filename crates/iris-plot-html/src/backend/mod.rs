use iris::render::RenderBackend;

use crate::{Plot, PlotError, plot::PlotPayload};

mod script_data;

const HEADER: &str = include_str!("page.html");
const ENGINE: &str = include_str!("canvas.js");

/// Render one plot into reusable standalone HTML storage.
#[derive(Debug, Default)]
pub struct HtmlPlotBackend {
    document: Vec<u8>,
}

impl RenderBackend<Plot<'_>> for HtmlPlotBackend {
    type Error = PlotError;
    type Frame<'a> = &'a [u8];

    fn render<'a>(&'a mut self, plot: &Plot<'_>) -> Result<Self::Frame<'a>, Self::Error> {
        self.document.clear();
        self.document.extend_from_slice(HEADER.as_bytes());
        serde_json::to_writer(
            script_data::Escaped(&mut self.document),
            &PlotPayload::from(plot),
        )?;
        self.document.extend_from_slice(b"</script>\n<script>\n");
        self.document.extend_from_slice(ENGINE.as_bytes());
        self.document.extend_from_slice(b"\n</script>\n");
        Ok(&self.document)
    }
}
