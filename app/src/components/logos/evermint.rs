use leptos::prelude::*;

#[component]
pub fn LogoEverMint(width: u32, height: u32) -> impl IntoView {
    view! {
        <svg width=width height=height viewBox="0 0 645 645" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect width="645" height="645" rx="110" fill="url(#paint0_radial_101_37)" />
            <ellipse cx="322.5" cy="321.723" rx="209.042" ry="203.602" fill="#222222" />
            <path
                d="M361.871 291.134C364.519 292.942 366.108 295.89 366.144 299.058L366.899 366.342C366.989 374.408 357.682 379.252 350.941 374.648L219.94 285.182C215.379 282.067 214.297 275.908 217.524 271.427L249.749 226.665C252.975 222.184 259.288 221.075 263.849 224.19L361.871 291.134Z"
                fill="url(#paint1_linear_101_37)"
            />
            <path
                d="M355.947 399.597C358.673 401.497 360.244 404.603 360.138 407.887L358.101 471.14C357.848 479.014 348.73 483.521 342.148 479.026L166.962 359.383C162.401 356.268 161.319 350.11 164.545 345.628L196.715 300.944C199.963 296.432 206.332 295.344 210.893 298.522L355.947 399.597Z"
                fill="url(#paint2_linear_101_37)"
            />
            <path
                d="M454.947 246.014C457.739 247.962 459.313 251.173 459.124 254.535L455.613 316.786C455.175 324.548 446.164 328.874 439.67 324.439L274.104 211.367C269.543 208.252 268.461 202.093 271.688 197.612L303.855 152.931C307.104 148.418 313.476 147.331 318.037 150.512L454.947 246.014Z"
                fill="url(#paint3_linear_101_37)"
            />
            <defs>
                <radialGradient
                    id="paint0_radial_101_37"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientUnits="userSpaceOnUse"
                    gradientTransform="translate(322.5 322.5) rotate(90) scale(322.5)"
                >
                    <stop stop-color="#65C565" />
                    <stop offset="0.338542" stop-color="#65C565" stop-opacity="0.72" />
                    <stop offset="0.619792" stop-color="#65C565" stop-opacity="0.53" />
                    <stop offset="0.78125" stop-color="#65C565" stop-opacity="0.46" />
                    <stop offset="0.864583" stop-color="#65C565" stop-opacity="0.330476" />
                    <stop offset="1" stop-color="#65C565" stop-opacity="0" />
                </radialGradient>
                <linearGradient
                    id="paint1_linear_101_37"
                    x1="254.268"
                    y1="236.292"
                    x2="255.016"
                    y2="338.488"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#415B40" />
                    <stop offset="1" stop-color="#4FA04D" />
                </linearGradient>
                <linearGradient
                    id="paint2_linear_101_37"
                    x1="203.431"
                    y1="311.957"
                    x2="192.415"
                    y2="421.055"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#415B40" />
                    <stop offset="1" stop-color="#4FA04D" />
                </linearGradient>
                <linearGradient
                    id="paint3_linear_101_37"
                    x1="310.078"
                    y1="163.601"
                    x2="301.55"
                    y2="271.515"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#415B40" />
                    <stop offset="1" stop-color="#4FA04D" />
                </linearGradient>
            </defs>
        </svg>
    }
}
