"use strict";
(() => {
    const plot = JSON.parse(document.getElementById("data").textContent);
    const canvas = document.getElementById("plot");
    const context = canvas.getContext("2d");
    const status = document.getElementById("status");
    const title = document.getElementById("title");
    title.textContent = plot.title;
    document.title = plot.title;
    const finite = Number.isFinite;
    const display = value => finite(value) ? value.toPrecision(5) : "Unavailable";
    const lineColors = ["#235ea2", "#b63b43", "#14776b", "#8d4d13", "#644994"];
    const visible = plot.series?.map(() => true);
    if (plot.series?.length) {
        const legend = document.getElementById("legend");
        legend.hidden = false;
        plot.series.forEach((series, index) => {
            const label = document.createElement("label");
            const input = document.createElement("input");
            input.type = "checkbox";
            input.checked = true;
            input.style.accentColor = lineColors[index % lineColors.length];
            input.addEventListener("change", () => {
                visible[index] = input.checked;
                hover = undefined;
                queueDraw();
            });
            label.append(input, document.createTextNode(series.name));
            legend.append(label);
        });
    }
    const initialView = () => ({ zoom: 1, x: 0, y: 0, yaw: -0.6, pitch: 0.4 });
    let view = initialView();
    let queued = false;
    let drag;
    let area;
    let projected = [];
    let hover;

    function extent(values) {
        let min = Infinity;
        let max = -Infinity;
        for (const value of values) {
            if (!finite(value)) continue;
            if (value < min) min = value;
            if (value > max) max = value;
        }
        return min <= max ? [min, max] : [0, 1];
    }

    function expanded([min, max]) {
        const gap = Math.max(Math.abs(min) * Number.EPSILON, 0.5);
        return min === max ? [Math.max(-Number.MAX_VALUE, min - gap),
            Math.min(Number.MAX_VALUE, max + gap)] : [min, max];
    }

    function edges(centres) {
        if (centres.length === 1) return expanded([centres[0], centres[0]]);
        const boundaries = [];
        for (let i = 0; i < centres.length - 1; i++) {
            boundaries.push(centres[i] / 2 + centres[i + 1] / 2);
        }
        return [Math.max(-Number.MAX_VALUE, centres[0] - (boundaries[0] - centres[0])),
            ...boundaries, Math.min(Number.MAX_VALUE, centres.at(-1) + (centres.at(-1) - boundaries.at(-1)))];
    }

    function* lineValues(key) {
        for (const line of plot.series) yield* line[key];
    }

    function* coordinates(axis) {
        for (const point of plot.points) yield point[axis];
    }

    const xEdges = plot.kind === "heatmap" ? edges(plot.x) : [];
    const yEdges = plot.kind === "heatmap" ? edges(plot.y) : [];
    const xBase = plot.kind === "heatmap" ? extent(xEdges) :
        plot.kind === "lines" ? expanded(extent(lineValues("x"))) : [0, 1];
    const yBase = plot.kind === "heatmap" ? extent(yEdges) :
        plot.kind === "lines" ? expanded(extent(lineValues("y"))) : [0, 1];
    const scatterBounds = plot.kind === "scatter" ?
        [0, 1, 2].map(axis => extent(coordinates(axis))) : [];
    const scale = plot.scale ?? plot.markers?.scale;
    const colorBounds = scale ? extent(plot.values ?? plot.markers.values) : [0, 1];
    const prompt = plot.kind === "scatter" ?
        "Drag to rotate. Scroll to zoom." : "Drag to pan. Scroll to zoom.";

    function rgb([red, green, blue, opacity]) {
        return "rgba(" + red + "," + green + "," + blue + "," + opacity / 255 + ")";
    }

    function ratio(value, [min, max]) {
        const span = max - min;
        if (span === 0) return 0.5;
        if (finite(span)) return (value - min) / span;
        // Halve only on overflow so adjacent subnormal inputs stay distinct.
        return (value / 2 - min / 2) / (max / 2 - min / 2);
    }

    function interpolate(start, end, fraction) {
        return start * (1 - fraction) + end * fraction;
    }

    function color(value) {
        const fraction = Math.max(0, Math.min(1, ratio(value, colorBounds)));
        return rgb(scale.colors[Math.round(fraction * (scale.colors.length - 1))]);
    }

    function position(x, y) {
        return [area.left + (ratio(x, xBase) * view.zoom + view.x) * area.width,
            area.bottom - (ratio(y, yBase) * view.zoom + view.y) * area.height];
    }

    function axes() {
        context.font = "12px system-ui";
        context.strokeStyle = "#607186";
        context.fillStyle = "#223249";
        context.strokeRect(area.left, area.top, area.width, area.height);
        for (let tick = 0; tick <= 5; tick++) {
            const t = tick / 5;
            const x = area.left + t * area.width;
            const y = area.bottom - t * area.height;
            const xValue = interpolate(...xBase, (t - view.x) / view.zoom);
            const yValue = interpolate(...yBase, (t - view.y) / view.zoom);
            context.textAlign = "center";
            context.fillText(display(xValue), x, area.bottom + 17);
            context.textAlign = "right";
            context.fillText(display(yValue), area.left - 5, y + 4);
        }
        context.textAlign = "center";
        context.fillText(plot.labels[0], area.left + area.width / 2, area.bottom + 37);
        context.save();
        context.translate(15, area.top + area.height / 2);
        context.rotate(-Math.PI / 2);
        context.fillText(plot.labels[1], 0, 0);
        context.restore();
    }

    function scaleLegend() {
        if (!scale) return;
        const barX = area.right + 12;
        const gradient = context.createLinearGradient(0, area.bottom, 0, area.top);
        scale.colors.forEach((entry, i) => gradient.addColorStop(i / (scale.colors.length - 1), rgb(entry)));
        context.fillStyle = gradient;
        context.fillRect(barX, area.top, 12, area.height);
        context.fillStyle = "#223249";
        context.textAlign = "left";
        context.fillText(display(colorBounds[1]), barX + 17, area.top + 10);
        context.fillText(display(colorBounds[0]), barX + 17, area.bottom);
        context.fillText(scale.label, barX, area.bottom + 20);
    }

    function lines() {
        plot.series.forEach((line, index) => {
            if (!visible[index]) return;
            context.beginPath();
            context.strokeStyle = lineColors[index % lineColors.length];
            context.lineWidth = 1.6;
            let adjacent = false;
            for (let sample = 0; sample < line.x.length; sample++) {
                const x = line.x[sample];
                const y = line.y[sample];
                if (!finite(x) || !finite(y)) {
                    adjacent = false;
                    continue;
                }
                const [px, py] = position(x, y);
                if (adjacent) context.lineTo(px, py);
                else context.moveTo(px, py);
                adjacent = true;
            }
            context.stroke();
        });
    }

    function heatmap() {
        for (let row = 0; row < plot.y.length; row++) {
            for (let col = 0; col < plot.x.length; col++) {
                const value = plot.values[row * plot.x.length + col];
                if (!finite(value)) continue;
                const [left, bottom] = position(xEdges[col], yEdges[row]);
                const [right, top] = position(xEdges[col + 1], yEdges[row + 1]);
                if (right < area.left || left > area.right || bottom < area.top || top > area.bottom) continue;
                context.fillStyle = color(value);
                context.fillRect(left, top, right - left, bottom - top);
            }
        }
    }

    function scatter() {
        const sy = Math.sin(view.yaw);
        const cy = Math.cos(view.yaw);
        const sp = Math.sin(view.pitch);
        const cp = Math.cos(view.pitch);
        const size = Math.min(area.width, area.height) * 0.73 * view.zoom;
        // Compare half-spans to preserve physical aspect ratio across axes
        // and cover the full finite binary64 coordinate range.
        const spans = scatterBounds.map(([min, max]) => max / 2 - min / 2);
        const maxSpan = Math.max(...spans);
        projected.length = 0;
        plot.points.forEach((point, index) => {
            if (!point.every(finite)) return;
            const [x, y, z] = point.map((value, axis) =>
                maxSpan === 0 ? 0 : (ratio(value, scatterBounds[axis]) - 0.5) * (spans[axis] / maxSpan));
            const u = x * cy - y * sy;
            const v = x * sy + y * cy;
            projected.push({
                x: area.left + area.width / 2 + size * u,
                y: area.top + area.height / 2 - size * (z * cp - v * sp),
                depth: v * cp + z * sp,
                index,
            });
        });
        projected.sort((a, b) => a.depth - b.depth);
        const markers = plot.markers;
        for (const point of projected) {
            const value = markers.values?.[point.index];
            if (scale && !finite(value)) continue;
            context.fillStyle = scale ? color(value) : rgb(markers.color);
            context.beginPath();
            context.arc(point.x, point.y, markers.radius, 0, 2 * Math.PI);
            context.fill();
        }
    }

    function draw() {
        queued = false;
        const rect = canvas.getBoundingClientRect();
        const { width, height } = rect;
        if (!width || !height) return;
        const dpi = window.devicePixelRatio || 1;
        canvas.width = Math.round(width * dpi);
        canvas.height = Math.round(height * dpi);
        context.setTransform(dpi, 0, 0, dpi, 0, 0);
        context.clearRect(0, 0, width, height);
        const left = width < 520 ? 61 : 82;
        const right = Math.max(left + 1, width - (scale ? 91 : 20));
        context.font = "12px system-ui";
        const top = 32;
        const bottom = height - 49;
        area = { left, right, top, bottom, width: right - left, height: bottom - top };
        context.save();
        context.beginPath();
        context.rect(left, top, area.width, area.height);
        context.clip();
        if (plot.kind === "lines") lines();
        else if (plot.kind === "heatmap") heatmap();
        else scatter();
        context.restore();
        context.font = "12px system-ui";
        if (plot.kind !== "scatter") axes();
        if (plot.kind === "scatter") {
            context.fillStyle = "#223249";
            context.textAlign = "center";
            context.fillText(plot.labels.join(" · "), area.left + area.width / 2, area.bottom + 25);
        }
        scaleLegend();
        status.textContent = hover ?? prompt;
    }

    function queueDraw() {
        if (!queued) {
            queued = true;
            window.requestAnimationFrame(draw);
        }
    }

    function cell(axis, coordinate) {
        let lo = 0;
        let hi = axis.length - 1;
        if (coordinate < axis[lo] || coordinate > axis[hi]) return -1;
        while (hi - lo > 1) {
            const middle = Math.floor((hi + lo) / 2);
            if (coordinate >= axis[middle]) lo = middle;
            else hi = middle;
        }
        return lo;
    }

    function readHover(px, py) {
        if (!area || px < area.left || px > area.right || py < area.top || py > area.bottom) return undefined;
        if (plot.kind !== "scatter") {
            const x = interpolate(...xBase, ((px - area.left) / area.width - view.x) / view.zoom);
            const y = interpolate(...yBase, ((area.bottom - py) / area.height - view.y) / view.zoom);
            const xy = plot.labels[0] + " " + display(x) + " · " + plot.labels[1] + " " + display(y);
            if (plot.kind === "lines") {
                let nearest;
                let distance = 81;
                for (const [seriesIndex, series] of plot.series.entries()) {
                    if (!visible[seriesIndex]) continue;
                    series.x.forEach((sx, index) => {
                        const sy = series.y[index];
                        if (!finite(sx) || !finite(sy)) return;
                        const [cx, cy] = position(sx, sy);
                        const squared = (px - cx) ** 2 + (py - cy) ** 2;
                        if (squared < distance) {
                            nearest = series.name + " · " + display(sx) + " · " + display(sy);
                            distance = squared;
                        }
                    });
                }
                return nearest ?? xy;
            }
            const col = cell(xEdges, x);
            const row = cell(yEdges, y);
            if (col < 0 || row < 0) return xy;
            return xy + " · " + plot.scale.label + " " + display(plot.values[row * plot.x.length + col]);
        }
        const match = projected.findLast(point => (px - point.x) ** 2 + (py - point.y) ** 2 <= 81);
        if (!match) return undefined;
        const point = plot.points[match.index];
        const coords = point.map((value, axis) => plot.labels[axis] + " " + display(value)).join(" · ");
        return scale ? coords + " · " + scale.label + " " + display(plot.markers.values[match.index]) : coords;
    }

    function reset() {
        view = initialView();
        hover = undefined;
        queueDraw();
    }

    canvas.addEventListener("pointerdown", event => {
        canvas.setPointerCapture(event.pointerId);
        drag = { x: event.clientX, y: event.clientY };
    });
    canvas.addEventListener("pointermove", event => {
        if (drag && area) {
            const dx = event.clientX - drag.x;
            const dy = event.clientY - drag.y;
            if (plot.kind === "scatter") {
                view.yaw += dx * 0.009;
                view.pitch = Math.max(-Math.PI / 2, Math.min(Math.PI / 2, view.pitch + dy * 0.009));
            } else {
                view.x += dx / area.width;
                view.y -= dy / area.height;
            }
            drag = { x: event.clientX, y: event.clientY };
            hover = undefined;
            queueDraw();
        } else {
            const { left, top } = canvas.getBoundingClientRect();
            hover = readHover(event.clientX - left, event.clientY - top);
            status.textContent = hover ?? prompt;
        }
    });
    for (const event of ["pointerup", "pointercancel", "lostpointercapture"]) {
        canvas.addEventListener(event, () => { drag = undefined; });
    }
    canvas.addEventListener("pointerleave", () => {
        hover = undefined;
        status.textContent = prompt;
    });
    canvas.addEventListener("wheel", event => {
        event.preventDefault();
        if (!area) return;
        const zoom = Math.max(0.2, Math.min(40, view.zoom * Math.exp(-event.deltaY * 0.002)));
        if (plot.kind !== "scatter") {
            const rect = canvas.getBoundingClientRect();
            const x = (event.clientX - rect.left - area.left) / area.width;
            const y = (area.bottom - event.clientY + rect.top) / area.height;
            view.x = x - (x - view.x) * zoom / view.zoom;
            view.y = y - (y - view.y) * zoom / view.zoom;
        }
        view.zoom = zoom;
        hover = undefined;
        queueDraw();
    }, { passive: false });
    canvas.addEventListener("dblclick", reset);
    document.getElementById("reset").addEventListener("click", reset);
    new ResizeObserver(queueDraw).observe(canvas);
    queueDraw();
})();
