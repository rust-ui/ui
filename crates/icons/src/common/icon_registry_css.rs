// * This file was was generated automatically. Please DO NOT edit manually.

/// Static CSS for AArrowDownAnimate animation
pub static AARROWDOWN_CSS: &str = r#"/* Letter "A" animation - targeting specific paths */
[data-name="AArrowDownAnimate"]:hover path[d="M3.5 13h6"],
[data-name="AArrowDownAnimate"]:hover path[d="m2 16 4.5-9 4.5 9"] {
    animation: letterAnimate 0.3s ease-in-out;
}

/* Arrow animation - targeting specific paths with delay */
[data-name="AArrowDownAnimate"]:hover path[d="M18 7v9"],
[data-name="AArrowDownAnimate"]:hover path[d="m14 12 4 4 4-4"] {
    animation: arrowAnimate 0.3s ease-in-out 0.2s both;
}

@keyframes letterAnimate {
    0% { opacity: 0; transform: scale(0.8); }
    100% { opacity: 1; transform: scale(1); }
}

@keyframes arrowAnimate {
    0% { opacity: 0; transform: translateY(-10px); }
    100% { opacity: 1; transform: translateY(0); }
}"#;

/// Static CSS for AirVentAnimate animation
pub static AIRVENT_CSS: &str = r#"/* Air vent wind animation - first wind path (immediate) */
[data-name="AirVentAnimate"]:hover path[d="M18 17.5a2.5 2.5 0 1 1-4 2.03V12"] {
    animation: windAnimate1 0.5s ease-in-out;
}

/* Air vent wind animation - second wind path (delayed) */
[data-name="AirVentAnimate"]:hover path[d="M6.6 15.572A2 2 0 1 0 10 17v-5"] {
    animation: windAnimate2 0.5s ease-in-out 0.2s both;
}

@keyframes windAnimate1 {
    0% { opacity: 0; transform: scale(0.8); }
    100% { opacity: 1; transform: scale(1); }
}

@keyframes windAnimate2 {
    0% { opacity: 0; transform: scale(0.8); }
    100% { opacity: 1; transform: scale(1); }
}"#;

/// Static CSS for AlarmClockAnimate animation
pub static ALARMCLOCK_CSS: &str = r#"/* Primary alarm clock elements - clock face circle, hands, and bottom legs */
[data-name="AlarmClockAnimate"]:hover circle[cx="12"],
[data-name="AlarmClockAnimate"]:hover path[d="M12 9v4l2 2"],
[data-name="AlarmClockAnimate"]:hover path[d="M6.38 18.7 4 21"],
[data-name="AlarmClockAnimate"]:hover path[d="M17.64 18.67 20 21"] {
    animation: primaryAlarmShake 0.3s linear infinite;
}

/* Secondary alarm clock elements - top alarm bells with more intense movement */
[data-name="AlarmClockAnimate"]:hover path[d="M5 3 2 6"],
[data-name="AlarmClockAnimate"]:hover path[d="m22 6-3-3"] {
    animation: secondaryAlarmShake 0.3s linear infinite;
}

@keyframes primaryAlarmShake {
    0% { transform: translateX(-1px) translateY(-1.5px); }
    16.67% { transform: translateX(1px) translateY(-1.5px); }
    33.33% { transform: translateX(-1px) translateY(-1.5px); }
    50% { transform: translateX(1px) translateY(-1.5px); }
    66.67% { transform: translateX(-1px) translateY(-1.5px); }
    83.33% { transform: translateX(0) translateY(-1.5px); }
    100% { transform: translateX(-1px) translateY(-1.5px); }
}

@keyframes secondaryAlarmShake {
    0% { transform: translateX(-2px) translateY(-2.5px); }
    16.67% { transform: translateX(2px) translateY(-2.5px); }
    33.33% { transform: translateX(-2px) translateY(-2.5px); }
    50% { transform: translateX(2px) translateY(-2.5px); }
    66.67% { transform: translateX(-2px) translateY(-2.5px); }
    83.33% { transform: translateX(0) translateY(-2.5px); }
    100% { transform: translateX(-2px) translateY(-2.5px); }
}"#;

/// Static CSS for AlignCenterAnimate animation
pub static ALIGNCENTER_CSS: &str = r#"/* Align center animation - only the middle line wiggles to simulate alignment adjustment */
[data-name="AlignCenterAnimate"]:hover path[d="M17 12H7"] {
    animation: alignCenterWiggle 1s linear;
}

@keyframes alignCenterWiggle {
    0% { transform: translateX(0); }
    20% { transform: translateX(3px); }
    40% { transform: translateX(-3px); }
    60% { transform: translateX(2px); }
    80% { transform: translateX(-2px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for AngryAnimate animation
pub static ANGRY_CSS: &str = r#"[data-name="AngryAnimate"]:hover {
    animation: wiggleZoom 0.8s ease-in-out forwards;
    transform-origin: center;
}

@keyframes wiggleZoom {
    0% { transform: scale(1) rotate(0deg); }
    15% { transform: scale(1.1) rotate(-8deg); }
    30% { transform: scale(1.1) rotate(8deg); }
    45% { transform: scale(1.1) rotate(-6deg); }
    60% { transform: scale(1.1) rotate(6deg); }
    75% { transform: scale(1.1) rotate(-3deg); }
    90% { transform: scale(1.1) rotate(3deg); }
    100% { transform: scale(1.1) rotate(0deg); }
}"#;

/// Static CSS for AnnoyedAnimate animation
pub static ANNOYED_CSS: &str = r#"/* Face animation - scales the entire SVG */
[data-name="AnnoyedAnimate"] {
    transition: transform 0.2s ease-out;
}

[data-name="AnnoyedAnimate"]:hover {
    transform: scale(1.05);
}

/* Mouth animation - horizontal scale and slight vertical movement */
[data-name="AnnoyedAnimate"] path[d="M8 15h8"] {
    transition: transform 0.2s ease-out;
}

[data-name="AnnoyedAnimate"]:hover path[d="M8 15h8"] {
    transform: scaleX(0.8) translateY(1px);
}

/* Left eyebrow animation - rotation and translation */
[data-name="AnnoyedAnimate"] path[d="M8 9h2"] {
    transition: transform 0.25s ease-out;
    transform-origin: center;
}

[data-name="AnnoyedAnimate"]:hover path[d="M8 9h2"] {
    transform: rotate(15deg) translate(-0.5px, -1px);
}

/* Right eyebrow animation - rotation and translation */
[data-name="AnnoyedAnimate"] path[d="M14 9h2"] {
    transition: transform 0.25s ease-out 0.05s;
    transform-origin: center;
}

[data-name="AnnoyedAnimate"]:hover path[d="M14 9h2"] {
    transform: rotate(15deg) translate(0.5px, -1px);
}"#;

/// Static CSS for ArchiveAnimate animation
pub static ARCHIVE_CSS: &str = r#"/* Archive lid lifting animation - smooth transition */
[data-name="ArchiveAnimate"] rect[width="20"][height="5"][x="2"][y="3"][rx="1"] {
    transition: transform 0.2s ease-out;
}

[data-name="ArchiveAnimate"]:hover rect[width="20"][height="5"][x="2"][y="3"][rx="1"] {
    transform: translateY(-1.5px);
}

/* Archive body opening animation - path morphing with reverse */
[data-name="ArchiveAnimate"] path[d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"] {
    animation-duration: 0.2s;
    animation-timing-function: ease-out;
    animation-fill-mode: both;
}

[data-name="ArchiveAnimate"]:hover path[d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"] {
    animation-name: archiveBodyOpen;
}

[data-name="ArchiveAnimate"]:not(:hover) path[d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"] {
    animation-name: archiveBodyClose;
}

/* Archive handle moving animation - path morphing with reverse */
[data-name="ArchiveAnimate"] path[d="M10 12h4"] {
    animation-duration: 0.2s;
    animation-timing-function: ease-out;
    animation-fill-mode: both;
}

[data-name="ArchiveAnimate"]:hover path[d="M10 12h4"] {
    animation-name: archiveHandleMove;
}

[data-name="ArchiveAnimate"]:not(:hover) path[d="M10 12h4"] {
    animation-name: archiveHandleReturn;
}


@keyframes archiveBodyOpen {
    0% { d: path('M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8'); }
    100% { d: path('M4 11v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V11'); }
}

@keyframes archiveBodyClose {
    0% { d: path('M4 11v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V11'); }
    100% { d: path('M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8'); }
}

@keyframes archiveHandleMove {
    0% { d: path('M10 12h4'); }
    100% { d: path('M10 15h4'); }
}

@keyframes archiveHandleReturn {
    0% { d: path('M10 15h4'); }
    100% { d: path('M10 12h4'); }
}"#;

/// Static CSS for ArrowBigDownAnimate animation
pub static ARROWBIGDOWN_CSS: &str = r#"[data-name="ArrowBigDownAnimate"]:hover path[d="M15 11a1 1 0 0 0 1 1h2.939a1 1 0 0 1 .75 1.811l-6.835 6.836a1.207 1.207 0 0 1-1.707 0L4.31 13.81a1 1 0 0 1 .75-1.811H8a1 1 0 0 0 1-1V5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1z"] {
    animation: arrowBigDownAnimate 0.4s ease-in-out;
}

@keyframes arrowBigDownAnimate {
    0% { transform: translateY(0); }
    50% { transform: translateY(2px); }
    100% { transform: translateY(0); }
}"#;

/// Static CSS for ArrowBigDownDashAnimate animation
pub static ARROWBIGDOWNDASH_CSS: &str = r#"/* Arrow body animation - targeting main path */
[data-name="ArrowBigDownDashAnimate"]:hover path[d="M15 11a1 1 0 0 0 1 1h2.939a1 1 0 0 1 .75 1.811l-6.835 6.836a1.207 1.207 0 0 1-1.707 0L4.31 13.81a1 1 0 0 1 .75-1.811H8a1 1 0 0 0 1-1V9a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1z"] {
    animation: arrowAnimate 0.4s ease-in-out;
}

/* Dash animation - targeting dash path */
[data-name="ArrowBigDownDashAnimate"]:hover path[d="M9 4h6"] {
    animation: dashAnimate 0.4s ease-in-out;
}

@keyframes arrowAnimate {
    0% { transform: translateY(0); }
    50% { transform: translateY(3px); }
    100% { transform: translateY(0); }
}

@keyframes dashAnimate {
    0% { transform: translateY(0); }
    50% { transform: translateY(1px); }
    100% { transform: translateY(0); }
}"#;

/// Static CSS for ArrowBigLeftAnimate animation
pub static ARROWBIGLEFT_CSS: &str = r#"[data-name="ArrowBigLeftAnimate"]:hover path[d="M13 9a1 1 0 0 1-1-1V5.061a1 1 0 0 0-1.811-.75l-6.835 6.836a1.207 1.207 0 0 0 0 1.707l6.835 6.835a1 1 0 0 0 1.811-.75V16a1 1 0 0 1 1-1h6a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1z"] {
    animation: arrowBigLeftAnimate 0.4s ease-in-out;
}

@keyframes arrowBigLeftAnimate {
    0% { transform: translateX(0); }
    50% { transform: translateX(-2px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for ArrowBigLeftDashAnimate animation
pub static ARROWBIGLEFTDASH_CSS: &str = r#"/* Arrow body animation - targeting main path */
[data-name="ArrowBigLeftDashAnimate"]:hover path[d="M13 9a1 1 0 0 1-1-1V5.061a1 1 0 0 0-1.811-.75l-6.835 6.836a1.207 1.207 0 0 0 0 1.707l6.835 6.835a1 1 0 0 0 1.811-.75V16a1 1 0 0 1 1-1h2a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1z"] {
    animation: arrowAnimate 0.4s ease-in-out;
}

/* Dash animation - targeting dash path */
[data-name="ArrowBigLeftDashAnimate"]:hover path[d="M20 9v6"] {
    animation: dashAnimate 0.4s ease-in-out;
}

@keyframes arrowAnimate {
    0% { transform: translateX(0); }
    50% { transform: translateX(-3px); }
    100% { transform: translateX(0); }
}

@keyframes dashAnimate {
    0% { transform: translateX(0); }
    50% { transform: translateX(-1px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for BlocksAnimate animation
pub static BLOCKS_CSS: &str = r#"/* Blocks Animation - Animated block moves diagonally up-left and down-right */

/* Initial state - smooth bounce back when not hovered */
[data-name="BlocksAnimate"] rect[x="14"][y="2"][width="8"][height="8"] {
    animation: blockRevert 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

/* On hover - move to new position */
[data-name="BlocksAnimate"]:hover rect[x="14"][y="2"][width="8"][height="8"] {
    animation: blockMove 0.3s ease-in-out forwards;
}

@keyframes blockMove {
    0% {
        transform: translateX(0) translateY(0);
    }
    100% {
        transform: translateX(-4px) translateY(4px);
    }
}

@keyframes blockRevert {
    0% {
        transform: translateX(-4px) translateY(4px);
    }
    100% {
        transform: translateX(0) translateY(0);
    }
}"#;

/// Static CSS for CalendarDaysAnimate animation
pub static CALENDARDAYS_CSS: &str = r#"/* Calendar Days Animation - Sequential opacity pulse for calendar day dots */

/* First row dots with staggered delays */
[data-name="CalendarDaysAnimate"]:hover path[d="M8 14h.01"] {
    animation: dotPulse 0.4s ease-in-out 0s;
}

[data-name="CalendarDaysAnimate"]:hover path[d="M12 14h.01"] {
    animation: dotPulse 0.4s ease-in-out 0.1s;
}

[data-name="CalendarDaysAnimate"]:hover path[d="M16 14h.01"] {
    animation: dotPulse 0.4s ease-in-out 0.2s;
}

/* Second row dots with continued staggered delays */
[data-name="CalendarDaysAnimate"]:hover path[d="M8 18h.01"] {
    animation: dotPulse 0.4s ease-in-out 0.3s;
}

[data-name="CalendarDaysAnimate"]:hover path[d="M12 18h.01"] {
    animation: dotPulse 0.4s ease-in-out 0.4s;
}

[data-name="CalendarDaysAnimate"]:hover path[d="M16 18h.01"] {
    animation: dotPulse 0.4s ease-in-out 0.5s;
}

@keyframes dotPulse {
    0% { opacity: 1; }
    50% { opacity: 0.3; }
    100% { opacity: 1; }
}"#;

/// Static CSS for CompassAnimate animation
pub static COMPASS_CSS: &str = r#"/* Compass Animation - Needle Rotation Effect */

/* Base state with smooth transition for the compass needle */
[data-name="CompassAnimate"] path[d="m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"] {
    transition: transform 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    transform-origin: center;
}

/* Hover state with 360 degree rotation for the compass needle */
[data-name="CompassAnimate"]:hover path[d="m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"] {
    transform: rotate(360deg);
}"#;

/// Static CSS for ExpandAnimate animation
pub static EXPAND_CSS: &str = r#"/* Expand icon - Corner arrows move outward on hover with spring-like transition */

/* Base state with spring-like transition for smooth reverse animation */
[data-name="ExpandAnimate"] path[d="m15 15 6 6"],
[data-name="ExpandAnimate"] path[d="M21 16v5h-5"] {
    transition: transform 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="ExpandAnimate"] path[d="m15 9 6-6"],
[data-name="ExpandAnimate"] path[d="M21 8V3h-5"] {
    transition: transform 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="ExpandAnimate"] path[d="M3 16v5h5"],
[data-name="ExpandAnimate"] path[d="m3 21 6-6"] {
    transition: transform 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="ExpandAnimate"] path[d="M3 8V3h5"],
[data-name="ExpandAnimate"] path[d="M9 9 3 3"] {
    transition: transform 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

/* Hover states - each corner moves in its respective direction */
/* Bottom-right corner moves down and right */
[data-name="ExpandAnimate"]:hover path[d="m15 15 6 6"],
[data-name="ExpandAnimate"]:hover path[d="M21 16v5h-5"] {
    transform: translate(2px, 2px);
}

/* Top-right corner moves up and right */
[data-name="ExpandAnimate"]:hover path[d="m15 9 6-6"],
[data-name="ExpandAnimate"]:hover path[d="M21 8V3h-5"] {
    transform: translate(2px, -2px);
}

/* Bottom-left corner moves down and left */
[data-name="ExpandAnimate"]:hover path[d="M3 16v5h5"],
[data-name="ExpandAnimate"]:hover path[d="m3 21 6-6"] {
    transform: translate(-2px, 2px);
}

/* Top-left corner moves up and left */
[data-name="ExpandAnimate"]:hover path[d="M3 8V3h5"],
[data-name="ExpandAnimate"]:hover path[d="M9 9 3 3"] {
    transform: translate(-2px, -2px);
}"#;

/// Static CSS for FrameAnimate animation
pub static FRAME_CSS: &str = r#"/* Base state with transition for smooth animations */
[data-name="FrameAnimate"] line[y1="6"] {
    transition: transform 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="FrameAnimate"] line[y1="18"] {
    transition: transform 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="FrameAnimate"] line[x1="6"] {
    transition: transform 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

[data-name="FrameAnimate"] line[x1="18"] {
    transition: transform 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

/* Hover state transforms */
[data-name="FrameAnimate"]:hover line[y1="6"] {
    transform: translateY(-4px);
}

[data-name="FrameAnimate"]:hover line[y1="18"] {
    transform: translateY(4px);
}

[data-name="FrameAnimate"]:hover line[x1="6"] {
    transform: translateX(-4px);
}

[data-name="FrameAnimate"]:hover line[x1="18"] {
    transform: translateX(4px);
}"#;

/// Static CSS for HeartAnimate animation
pub static HEART_CSS: &str = r#"[data-name="HeartAnimate"]:hover {
    animation: HeartAnimation 1.2s ease-in-out;
}

@keyframes HeartAnimation {
    0% { transform: scale(1); }
    14% { transform: scale(1.15); }
    28% { transform: scale(1); }
    42% { transform: scale(1.08); }
    56% { transform: scale(1); }
    100% { transform: scale(1); }
}"#;

/// Static CSS for PanelLeftOpenAnimate animation
pub static PANELLEFTOPEN_CSS: &str = r#"/* Panel Left Open Animation - Chevron slide animation */
[data-name="PanelLeftOpenAnimate"]:hover path[d="m14 9 3 3-3 3"] {
    animation: chevronSlideRight 0.5s ease-in-out;
}

@keyframes chevronSlideRight {
    0% { transform: translateX(0); }
    40% { transform: translateX(1.5px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for PlusAnimate animation
pub static PLUS_CSS: &str = r#"[data-name="PlusAnimate"] {
    transition: transform 0.7s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

[data-name="PlusAnimate"]:hover {
    transform: rotate(180deg);
}"#;

/// Static CSS for SearchAnimate animation
pub static SEARCH_CSS: &str = r#"/* Base state with smooth transition for search movement */
[data-name="SearchAnimate"] {
    transition: transform 1s cubic-bezier(0.68, -0.3, 0.32, 1.3);
    transform-origin: center;
}

/* Hover state triggers search scanning animation */
[data-name="SearchAnimate"]:hover {
    animation: searchScan 1s cubic-bezier(0.68, -0.3, 0.32, 1.3);
}

@keyframes searchScan {
    0% { transform: translateX(0) translateY(0); }
    25% { transform: translateX(0) translateY(-4px); }
    50% { transform: translateX(-3px) translateY(0); }
    100% { transform: translateX(0) translateY(0); }
}"#;

/// Static CSS for SquareChevronRightAnimate animation
pub static SQUARECHEVRONRIGHT_CSS: &str = r#"/* Chevron bounce animation */
[data-name="SquareChevronRightAnimate"]:hover path[d="m10 8 4 4-4 4"] {
    animation: chevronBounce 0.5s ease-in-out;
}

@keyframes chevronBounce {
    0% { transform: translateX(0); }
    40% { transform: translateX(2px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for TornadoAnimate animation
pub static TORNADO_CSS: &str = r#"/* Tornado lines - sequential horizontal sway animation */
[data-name="TornadoAnimate"]:hover path[d="M21 4H3"] {
    animation: tornadoSway1 0.6s ease-in-out 0.1s;
}

[data-name="TornadoAnimate"]:hover path[d="M18 8H6"] {
    animation: tornadoSway2 0.6s ease-in-out 0.2s;
}

[data-name="TornadoAnimate"]:hover path[d="M19 12H9"] {
    animation: tornadoSway3 0.6s ease-in-out 0.3s;
}

[data-name="TornadoAnimate"]:hover path[d="M16 16h-6"] {
    animation: tornadoSway4 0.6s ease-in-out 0.4s;
}

[data-name="TornadoAnimate"]:hover path[d="M11 20H9"] {
    animation: tornadoSway5 0.6s ease-in-out 0.5s;
}

@keyframes tornadoSway1 {
    0% { transform: translateX(0); }
    50% { transform: translateX(1px); }
    100% { transform: translateX(0); }
}

@keyframes tornadoSway2 {
    0% { transform: translateX(0); }
    50% { transform: translateX(2px); }
    100% { transform: translateX(0); }
}

@keyframes tornadoSway3 {
    0% { transform: translateX(0); }
    50% { transform: translateX(3px); }
    100% { transform: translateX(0); }
}

@keyframes tornadoSway4 {
    0% { transform: translateX(0); }
    50% { transform: translateX(4px); }
    100% { transform: translateX(0); }
}

@keyframes tornadoSway5 {
    0% { transform: translateX(0); }
    50% { transform: translateX(5px); }
    100% { transform: translateX(0); }
}"#;

/// Static CSS for WindAnimate animation
pub static WIND_CSS: &str = r#"/* Wind animation - path drawing with different delays */

/* Base state - all wind paths visible */
[data-name="WindAnimate"] path[d*="M12.8 19.6A2 2 0 1 0 14 16H2"],
[data-name="WindAnimate"] path[d*="M17.5 8a2.5 2.5 0 1 1 2 4H2"],
[data-name="WindAnimate"] path[d*="M9.8 4.4A2 2 0 1 1 11 8H2"] {
    stroke-dasharray: 25;
    stroke-dashoffset: 0;
    opacity: 1;
}

/* Animation on hover - path drawing with staggered delays */
/* Path 2 (custom: 0) - no delay */
[data-name="WindAnimate"]:hover path[d*="M17.5 8a2.5 2.5 0 1 1 2 4H2"] {
    animation: windPathDraw 0.5s ease-in-out both;
}

/* Path 1 (custom: 0.2) - 0.2s delay */
[data-name="WindAnimate"]:hover path[d*="M12.8 19.6A2 2 0 1 0 14 16H2"] {
    animation: windPathDraw 0.5s ease-in-out 0.2s both;
}

/* Path 3 (custom: 0.4) - 0.4s delay */
[data-name="WindAnimate"]:hover path[d*="M9.8 4.4A2 2 0 1 1 11 8H2"] {
    animation: windPathDraw 0.5s ease-in-out 0.4s both;
}

@keyframes windPathDraw {
    0% {
        opacity: 0;
        stroke-dasharray: 25;
        stroke-dashoffset: 25;
    }
    100% {
        opacity: 1;
        stroke-dasharray: 25;
        stroke-dashoffset: 0;
    }
}"#;
