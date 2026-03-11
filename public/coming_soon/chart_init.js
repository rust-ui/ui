// ApexCharts initializer for Leptos SPA
// Problem: In SPAs, external <script src="..."> tags don't re-execute on route changes.
// Solution: Use MutationObserver to detect new/changed charts and dynamically load ApexCharts.
(() => {
  // Guard: if this script is somehow loaded twice (e.g. once from ChartsLayout and once from
  // ThemesBlocks), bail early so only one MutationObserver and one chartInstances Map exist.
  if (window.__chartInitDone) return;
  window.__chartInitDone = true;
  const chartInstances = new Map();
  const chartMetadata = new Map();
  let currentTheme = document.documentElement.classList.contains("dark")
    ? "dark"
    : "light";
  let currentColorTheme = document.documentElement.dataset.colorTheme || "";
  // Track the actual computed --chart-1 value so we detect inline CSS var
  // overrides (injected by Rust/WASM color-theme picker) even when
  // data-color-theme hasn't changed yet or the attribute change was missed.
  let lastChart1Value = getComputedStyle(document.documentElement).getPropertyValue("--chart-1").trim();
  let apexChartsLoading = false;

  // Dynamically load ApexCharts if not already loaded
  function loadApexCharts() {
    if (typeof ApexCharts !== "undefined" || apexChartsLoading) return Promise.resolve();
    apexChartsLoading = true;

    return new Promise((resolve, reject) => {
      const script = document.createElement("script");
      script.src = "/cdn/apexcharts.5.3.6.min.js";
      script.onload = () => {
        apexChartsLoading = false;
        resolve();
      };
      script.onerror = () => {
        apexChartsLoading = false;
        reject(new Error("Failed to load ApexCharts"));
      };
      document.head.appendChild(script);
    });
  }

  function initChart(container) {
    // Auto-assign a stable unique ID if the container has none (prevents duplicate-ID conflicts
    // when the same component renders twice for responsive show/hide, e.g. mobile + desktop).
    if (!container.id) {
      container.id = 'apex-auto-' + Math.random().toString(36).slice(2, 9);
    }
    const containerId = container.id;
    const chartName = container.dataset.name;
    const chartValues = container.dataset.chartValues;
    const chartLabels = container.dataset.chartLabels;

    // Convert PascalCase chart name to lowercase type (e.g., "AreaChart" -> "area")
    const chartType = chartName ? chartName.replace(/Chart$/, '').toLowerCase() : null;

    // Destroy existing chart before creating new one
    if (chartInstances.has(containerId)) {
      chartInstances.get(containerId).destroy();
      chartInstances.delete(containerId);
    }

    const CHART_DATA = JSON.parse(chartValues || "[]");
    const CHART_LABELS = JSON.parse(chartLabels || "[]");

    // Get CSS colors from design system
    const CSS_VARS = {
      chart1: "--chart-1",
      chart2: "--chart-2",
      chart3: "--chart-3",
      chart4: "--chart-4",
      chart5: "--chart-5",
      chartPrimaryFallback: "--color-chart-primary",
      chartMutedFallback: "--color-chart-muted",
      foreground: "--color-foreground",
      mutedForeground: "--color-muted-foreground",
      chartGrid: "--color-chart-grid",
    };
    const styles = getComputedStyle(document.documentElement);
    const get = (v) => styles.getPropertyValue(v).trim();
    const colors = {
      chartPrimary: resolveColor(get(CSS_VARS.chart1)) || resolveColor(get(CSS_VARS.chartPrimaryFallback)),
      chartMuted: resolveColor(get(CSS_VARS.chart2)) || resolveColor(get(CSS_VARS.chartMutedFallback)),
      chart3: resolveColor(get(CSS_VARS.chart3)),
      chart4: resolveColor(get(CSS_VARS.chart4)),
      chart5: resolveColor(get(CSS_VARS.chart5)),
      foregroundColor: get(CSS_VARS.foreground),
      mutedForegroundColor: get(CSS_VARS.mutedForeground),
      gridColor: get(CSS_VARS.chartGrid),
    };

    // Build options using appropriate builder function
    const chartBuilders = {
      radar: () => buildRadarChartOptions(CHART_DATA, CHART_LABELS, colors, currentTheme),
      bar: () => buildBarChartOptions(CHART_DATA, CHART_LABELS, colors, container, currentTheme),
      area: () => buildAreaChartOptions(CHART_DATA, CHART_LABELS, colors, container, currentTheme),
      line: () => buildLineChartOptions(CHART_DATA, CHART_LABELS, colors, container, currentTheme),
      pie: () => buildPieChartOptions(CHART_DATA, CHART_LABELS, colors, currentTheme),
      radial: () => buildRadialChartOptions(CHART_DATA, CHART_LABELS, colors, currentTheme),
    };

    const options = chartBuilders[chartType]?.();

    if (options && typeof ApexCharts !== "undefined") {
      const chart = new ApexCharts(container, options);
      chart.render().then(() => {
        const annotationsAttr = container.dataset.chartAnnotations;
        if (!annotationsAttr) return;
        const allAnnotations = JSON.parse(annotationsAttr);
        const labelsAttr = container.dataset.chartLabels;
        if (!labelsAttr) return;
        const labels = JSON.parse(labelsAttr);
        const matching = allAnnotations.filter(a => labels.includes(a.x));
        if (matching.length === 0) return;

        const svg = container.querySelector(".apexcharts-svg");
        if (!svg) return;

        const inner = svg.querySelector(".apexcharts-inner");
        const innerTransform = inner?.getAttribute("transform") || "";
        const innerOffsetX = parseFloat(innerTransform.match(/translate\(([^,]+)/)?.[1] || 0);
        const innerOffsetY = parseFloat(innerTransform.match(/translate\([^,]+,\s*([^)]+)/)?.[1] || 0);

        const firstGridLine = svg.querySelector(".apexcharts-gridlines-horizontal line");
        const gridY1 = parseFloat(firstGridLine?.getAttribute("y1") || 0) + innerOffsetY;
        const axisLabelY = parseFloat(svg.querySelector(".apexcharts-xaxis-texts-g text")?.getAttribute("y") || 120);
        const gridY2 = axisLabelY - 10;

        const xAxisTexts = Array.from(svg.querySelectorAll(".apexcharts-xaxis-texts-g text"));
        const dateToX = {};
        labels.forEach(dateStr => {
          const formatted = new Date(dateStr).toLocaleDateString("en-US", { month: "short", day: "numeric" });
          const el = xAxisTexts.find(t => t.textContent.includes(formatted));
          if (el) dateToX[dateStr] = parseFloat(el.getAttribute("x")) + innerOffsetX;
        });

        let tooltip = document.getElementById("article-annotation-tooltip");
        if (!tooltip) {
          tooltip = document.createElement("div");
          tooltip.id = "article-annotation-tooltip";
          tooltip.style.cssText = "position:fixed;background:#1e293b;color:#fff;padding:6px 10px;border-radius:6px;font-size:12px;pointer-events:none;z-index:9999;display:none;max-width:260px;line-height:1.4;box-shadow:0 4px 12px rgba(0,0,0,0.3)";
          document.body.appendChild(tooltip);
        }

        const ns = "http://www.w3.org/2000/svg";
        const annotGroup = document.createElementNS(ns, "g");
        annotGroup.setAttribute("class", "custom-article-annotations");

        matching.forEach(ann => {
          const x = dateToX[ann.x];
          if (x === undefined) return;

          const line = document.createElementNS(ns, "line");
          line.setAttribute("x1", x); line.setAttribute("x2", x);
          line.setAttribute("y1", gridY1); line.setAttribute("y2", gridY2);
          line.setAttribute("stroke", "#94a3b8"); line.setAttribute("stroke-width", "1");
          line.setAttribute("stroke-dasharray", "3,3");

          const diamond = document.createElementNS(ns, "text");
          diamond.setAttribute("x", x); diamond.setAttribute("y", gridY1 - 2);
          diamond.setAttribute("text-anchor", "middle");
          diamond.setAttribute("font-size", "10"); diamond.setAttribute("fill", "#6495ed");
          diamond.setAttribute("style", "cursor:pointer");
          diamond.textContent = "◆";

          [line, diamond].forEach(el => {
            el.addEventListener("mouseenter", e => {
              tooltip.textContent = ann.title;
              tooltip.style.display = "block";
              tooltip.style.left = (e.clientX + 12) + "px";
              tooltip.style.top = (e.clientY - 10) + "px";
            });
            el.addEventListener("mousemove", e => {
              tooltip.style.left = (e.clientX + 12) + "px";
              tooltip.style.top = (e.clientY - 10) + "px";
            });
            el.addEventListener("mouseleave", () => { tooltip.style.display = "none"; });
          });

          annotGroup.appendChild(line);
          annotGroup.appendChild(diamond);
        });

        svg.appendChild(annotGroup);
      });

      // Store instance for cleanup and metadata for change detection
      chartInstances.set(containerId, chart);
      const chartSeriesNames = container.dataset.chartSeriesNames;
      chartMetadata.set(containerId, {
        chartName,
        chartValues,
        chartLabels,
        chartSeriesNames,
      });
      container.dataset.chartInitialized = "true";
    }
  }

  async function checkAndInitCharts() {
    // Allow initialization when DOM is ready (interactive) or fully loaded (complete)
    if (document.readyState === "loading") return;

    const newTheme = document.documentElement.classList.contains("dark")
      ? "dark"
      : "light";
    const newColorTheme = document.documentElement.dataset.colorTheme || "";
    // Read the live computed value of --chart-1. Rust/WASM sets CSS vars as
    // inline styles on <html> before setting data-color-theme, so this catches
    // theme changes even if the attribute observation fires out of order.
    const newChart1Value = getComputedStyle(document.documentElement).getPropertyValue("--chart-1").trim();
    const themeChanged =
      currentTheme !== newTheme ||
      currentColorTheme !== newColorTheme ||
      newChart1Value !== lastChart1Value;
    if (themeChanged) {
      currentTheme = newTheme;
      currentColorTheme = newColorTheme;
      lastChart1Value = newChart1Value;
      for (const chart of chartInstances.values()) chart.destroy();
      chartInstances.clear();
      chartMetadata.clear();
    }

    for (const [containerId, chart] of chartInstances) {
      if (!document.getElementById(containerId)) {
        chart.destroy();
        chartInstances.delete(containerId);
        chartMetadata.delete(containerId);
      }
    }

    const containers = document.querySelectorAll("div[data-chart-values]");

    // If there are chart containers but ApexCharts isn't loaded, load it dynamically
    if (containers.length > 0 && typeof ApexCharts === "undefined") {
      await loadApexCharts();
    }

    containers.forEach((container) => {
      if (container.dataset.name && typeof ApexCharts !== "undefined") {
        const containerId = container.id;
        const currentMeta = chartMetadata.get(containerId);
        const chartName = container.dataset.name;
        const chartValues = container.dataset.chartValues;
        const chartLabels = container.dataset.chartLabels;
        const chartSeriesNames = container.dataset.chartSeriesNames;

        const needsInit =
          !container.dataset.chartInitialized ||
          !currentMeta ||
          currentMeta.chartName !== chartName ||
          currentMeta.chartValues !== chartValues ||
          currentMeta.chartLabels !== chartLabels ||
          currentMeta.chartSeriesNames !== chartSeriesNames;

        if (needsInit) initChart(container);
      }
    });
  }

  // Debounced MutationObserver: Leptos SPA navigation triggers many DOM mutations in rapid
  // succession. Without debouncing, checkAndInitCharts fires concurrently before
  // data-chart-initialized is set, causing the same chart to render multiple times.
  let debounceTimer;
  const observer = new MutationObserver(() => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(checkAndInitCharts, 50);
  });

  // Start observing when DOM is ready
  function startObserver() {
    // Watch body for new chart containers added by SPA navigation.
    observer.observe(document.body, { childList: true, subtree: true });
    // Watch <html> for:
    //   - data-color-theme: set by Rust after applying a color theme
    //   - style: Rust injects CSS vars (--chart-1 etc.) as inline styles on <html>
    //     before setting data-color-theme. Watching style ensures we never miss
    //     a color change even if the attribute mutation fires in an unexpected order.
    //   - class: catches dark/light mode toggles (adds/removes "dark" class)
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-color-theme", "style", "class"],
    });
    checkAndInitCharts();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", startObserver);
  } else {
    startObserver();
  }

  // Also check on full page load for CSS readiness
  window.addEventListener("load", checkAndInitCharts);

  // Handle window resize to update charts with new responsive tick amounts
  let resizeTimeout;
  window.addEventListener("resize", () => {
    clearTimeout(resizeTimeout);
    resizeTimeout = setTimeout(() => {
      // Clear metadata to force chart reinitialization with new dimensions
      chartMetadata.clear();
      checkAndInitCharts();
    }, 250);
  });
})();


/* ========================================================== */
/*                     FUNCTIONS                        */
/* ========================================================== */

// Resolves any CSS color value (including modern oklch/oklch) to rgb() format
// that ApexCharts can parse. The browser handles the conversion natively.
function resolveColor(cssValue) {
  if (!cssValue) return cssValue;
  const el = document.createElement("span");
  el.style.position = "fixed";
  el.style.color = cssValue;
  document.documentElement.appendChild(el);
  const resolved = getComputedStyle(el).color;
  el.remove();
  return resolved || cssValue;
}

function buildRadarChartOptions(data, labels, colors, theme) {
  return {
    series: [{ name: "Metrics", data }],
    chart: {
      type: "radar",
      height: 400,
      toolbar: { show: false },
      animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } },
    },
    colors: [colors.chartPrimary],
    stroke: { show: true, width: 2, colors: [colors.chartPrimary], dashArray: 0 },
    fill: { type: "solid", opacity: 0.3 },
    markers: { size: 4, colors: [colors.chartPrimary], strokeColor: colors.chartPrimary, strokeWidth: 2, hover: { size: 6 } },
    xaxis: { categories: labels, labels: { show: true, style: { colors: Array(labels.length).fill(colors.mutedForegroundColor), fontSize: "12px" } } },
    yaxis: { show: false },
    plotOptions: { radar: { size: 140, polygons: { strokeColor: colors.gridColor, strokeWidth: 1, connectorColors: colors.gridColor, fill: { colors: ["transparent"] } } } },
    tooltip: { theme, y: { formatter: (value) => `${value}%` }, followCursor: true },
    legend: { show: false },
  };
}

function buildBarChartOptions(data, labels, colors, container, theme) {
  const valuePrefix = container.dataset.chartValuePrefix || "";
  const showXAxis = container.dataset.chartShowXaxis !== "false";
  const showYAxis = container.dataset.chartShowYaxis !== "false";
  const showGrid = container.dataset.chartShowGrid === "true";
  const containerHeight = container.offsetHeight || 400;
  return {
    series: [{ name: "Value", data: data.map((value) => Math.abs(value)) }],
    chart: { type: "bar", height: containerHeight, toolbar: { show: false }, animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } } },
    colors: data.map((value) => (value < 0 ? colors.chartMuted : colors.chartPrimary)),
    plotOptions: { bar: { borderRadius: 4, borderRadiusApplication: "end", distributed: true, columnWidth: "60%" } },
    dataLabels: { enabled: false },
    legend: { show: false },
    xaxis: showXAxis
      ? { categories: labels, labels: { style: { colors: colors.mutedForegroundColor } }, axisBorder: { color: colors.gridColor } }
      : { categories: labels, labels: { show: false }, axisBorder: { show: false }, axisTicks: { show: false } },
    yaxis: showYAxis
      ? { labels: { style: { colors: colors.mutedForegroundColor }, formatter: (value) => `${valuePrefix}${value}` } }
      : { show: false },
    grid: showGrid ? { borderColor: colors.gridColor } : { show: false, padding: { top: -10, bottom: -10, left: -10, right: -10 } },
    tooltip: { theme, y: { formatter: (_value, { dataPointIndex }) => `${valuePrefix}${data[dataPointIndex].toLocaleString()}` }, followCursor: true, intersect: false },
  };
}

function buildLineChartOptions(data, labels, colors, container, theme) {
  const valuePrefix = container.dataset.chartValuePrefix || "";
  const showXAxis = container.dataset.chartShowXaxis !== "false";
  const showYAxis = container.dataset.chartShowYaxis !== "false";
  const showGrid = container.dataset.chartShowGrid === "true";
  const showMarkers = container.dataset.chartMarkers === "true";
  const sparkline = container.dataset.chartSparkline === "true";
  const isMultiDataset = Array.isArray(data[0]);
  const seriesNamesAttr = container.dataset.chartSeriesNames;
  const seriesNames = seriesNamesAttr ? JSON.parse(seriesNamesAttr) : null;
  const containerHeight = container.offsetHeight || 400;

  const series = isMultiDataset
    ? data.map((d, i) => ({ name: seriesNames?.[i] || `Series ${i + 1}`, data: d }))
    : [{ name: seriesNames?.[0] || "Value", data }];

  const seriesColors = isMultiDataset
    ? [colors.chartPrimary, colors.chartMuted]
    : [colors.chartPrimary];

  // Sparkline mode: minimal chrome, used for small inline charts (e.g. Total Revenue)
  if (sparkline) {
    return {
      series,
      chart: { type: "line", height: containerHeight, sparkline: { enabled: true }, toolbar: { show: false }, animations: { enabled: true, speed: 800 } },
      colors: seriesColors,
      stroke: { curve: "smooth", width: 2 },
      markers: showMarkers
        ? { size: 4, colors: ["#fff"], strokeColors: colors.chartPrimary, strokeWidth: 2, hover: { size: 6 } }
        : { size: 0 },
      tooltip: { theme, fixed: { enabled: false }, x: { show: false }, y: { formatter: (v) => `${valuePrefix}${v}` } },
    };
  }

  return {
    series,
    chart: { type: "line", height: containerHeight, toolbar: { show: false }, animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } } },
    colors: seriesColors,
    dataLabels: { enabled: false },
    stroke: { curve: "smooth", width: 2 },
    legend: {
      show: isMultiDataset && seriesNames !== null,
      position: "top",
      horizontalAlign: "left",
      fontSize: "12px",
      fontFamily: "inherit",
      labels: { colors: colors.foregroundColor },
      markers: { width: 8, height: 8, radius: 2 },
      itemMargin: { horizontal: 12 },
    },
    xaxis: showXAxis
      ? { categories: labels, labels: { style: { colors: colors.mutedForegroundColor }, rotate: 0 }, axisBorder: { color: colors.gridColor }, axisTicks: { show: false } }
      : { categories: labels, labels: { show: false }, axisBorder: { show: false }, axisTicks: { show: false } },
    yaxis: showYAxis
      ? { labels: { style: { colors: colors.mutedForegroundColor }, formatter: (v) => `${valuePrefix}${v}` } }
      : { show: false },
    grid: showGrid
      ? { borderColor: colors.gridColor, padding: { top: -5, bottom: 0 } }
      : { show: false, padding: { top: -5, bottom: 0 } },
    markers: showMarkers
      ? { size: 4, colors: ["#fff"], strokeColors: colors.chartPrimary, strokeWidth: 2, hover: { size: 6 } }
      : { size: 0, hover: { size: 4, sizeOffset: 3 } },
    tooltip: { theme, shared: true, intersect: false, followCursor: true, y: { formatter: (v) => `${valuePrefix}${v}` } },
  };
}

function buildPieChartOptions(data, labels, colors, theme) {
  const styles = getComputedStyle(document.documentElement);
  return {
    series: data,
    chart: { type: "pie", height: 400, animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } } },
    labels,
    colors: [colors.chartPrimary, colors.chartMuted, colors.foregroundColor, colors.gridColor, styles.getPropertyValue("--color-chart-5").trim()],
    dataLabels: { enabled: true, style: { colors: ["#fff"] }, dropShadow: { enabled: false } },
    legend: { position: "bottom", labels: { colors: colors.foregroundColor } },
    stroke: { width: 0 },
    tooltip: { theme, y: { formatter: (value) => `${value}%` } },
    states: { hover: { filter: { type: "lighten", value: 0.15 } }, active: { filter: { type: "darken", value: 0.15 } } },
  };
}

function buildRadialChartOptions(data, labels, colors, theme) {
  const styles = getComputedStyle(document.documentElement);
  return {
    series: data,
    chart: { type: "radialBar", height: 400, animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } } },
    plotOptions: {
      radialBar: {
        offsetY: 0, startAngle: 0, endAngle: 270,
        hollow: { margin: 5, size: "30%", background: "transparent" },
        dataLabels: {
          name: { show: true, color: colors.foregroundColor, fontSize: "14px", fontWeight: 500, offsetY: -10 },
          value: { show: true, color: colors.foregroundColor, fontSize: "22px", fontWeight: 600, formatter: (val) => `${val}%` },
          total: { show: true, label: "Total", color: colors.foregroundColor, fontSize: "14px", fontWeight: 500, formatter: () => `${Math.round(data.reduce((a, b) => a + b, 0) / data.length)}%` },
        },
        track: { background: colors.gridColor, strokeWidth: "97%", margin: 5 },
      },
    },
    colors: [colors.chartPrimary, `hsl(${styles.getPropertyValue("--color-chart-secondary").trim()})`, `hsl(${styles.getPropertyValue("--color-chart-tertiary").trim()})`, colors.chartMuted],
    labels,
    legend: { show: true, floating: true, fontSize: "14px", position: "left", offsetX: 0, offsetY: 15, labels: { colors: colors.foregroundColor, useSeriesColors: false }, markers: { size: 0 }, formatter: (seriesName, opts) => `${seriesName}: ${opts.w.globals.series[opts.seriesIndex]}%`, itemMargin: { vertical: 3 } },
    tooltip: { theme, enabled: true, y: { formatter: (val) => `${val}%` } },
  };
}

function buildAreaChartOptions(data, labels, colors, container, theme) {
  const chart1Color = colors.chartPrimary || "#6495ed";
  const chart2Color = colors.chartMuted || "#add8ff";
  const chart3Color = colors.chart3 || "#93c5fd";
  const isMultiDataset = Array.isArray(data[0]);
  const seriesNamesAttr = container.dataset.chartSeriesNames;
  const seriesNames = seriesNamesAttr ? JSON.parse(seriesNamesAttr) : null;
  const curveType = container.dataset.chartCurve?.toLowerCase() || "smooth";
  const stackType = container.dataset.chartStackType || "normal";
  const gradientEnabled = container.dataset.chartGradient === "true";
  const showYAxis = container.dataset.chartShowYaxis !== "false";
  const showGrid = container.dataset.chartShowGrid === "true";
  const valuePrefix = container.dataset.chartValuePrefix || "";
  const series = isMultiDataset ? data.map((d, index) => ({ name: seriesNames?.[index] || `Series ${index + 1}`, data: d })) : [{ name: "Revenue", data }];
  const containerHeight = container.offsetHeight || 400;
  const containerWidth = container.offsetWidth || 400;

  // Responsive tick amount based on container width
  const tickAmount = containerWidth < 640 ? 4 : containerWidth < 768 ? 6 : 12;

  return {
    series,
    chart: { type: "area", height: containerHeight, toolbar: { show: false }, animations: { enabled: true, speed: 800, animateGradually: { enabled: true, delay: 150 } }, stacked: true, stackType: stackType === "100%" ? "100%" : undefined },
    plotOptions: { area: { stacked: true } },
    colors: isMultiDataset ? [chart1Color, chart2Color, chart3Color] : [colors.chartPrimary],
    dataLabels: { enabled: false },
    stroke: { curve: curveType, width: 2 },
    fill: gradientEnabled ? { type: "gradient", gradient: { shadeIntensity: 0.1, opacityFrom: 0.5, opacityTo: 0.2, stops: [0, 90, 100] } } : { type: "solid", opacity: 0.4 },
    legend: { show: seriesNames !== null, position: "bottom", horizontalAlign: "center", fontSize: "12px", fontFamily: "inherit", fontWeight: 500, offsetY: 5, markers: { width: 8, height: 8, radius: 2 }, itemMargin: { horizontal: 12, vertical: 0 }, labels: { colors: colors.foregroundColor } },
    xaxis: {
      categories: labels,
      labels: {
        style: { colors: colors.mutedForegroundColor }, rotate: 0,
        formatter: (value) => {
          if (value?.match?.(/^\d{4}-\d{2}-\d{2}$/)) {
            return new Date(value).toLocaleDateString("en-US", { month: "short", day: "numeric" });
          }
          return value;
        },
        showDuplicates: false, hideOverlappingLabels: true,
      },
      tickAmount, axisBorder: { color: colors.gridColor }, axisTicks: { show: false }, crosshairs: { show: false }, tooltip: { enabled: false },
    },
    yaxis: showYAxis ? { labels: { style: { colors: colors.mutedForegroundColor }, formatter: (value) => `${valuePrefix}${value}` } } : { show: false },
    grid: showGrid ? { borderColor: colors.gridColor, padding: { top: -5, bottom: 0 } } : { show: false, padding: { top: -5, bottom: 0 } },
    markers: { size: 0, hover: { size: 6, sizeOffset: 3 } },
    tooltip: { theme, y: { formatter: (value) => `${valuePrefix}${value.toLocaleString()}` }, followCursor: true, intersect: false },
  };
}
