use leptos::prelude::*;

#[component]
pub fn LogoEverUI(width: u32, height: u32) -> impl IntoView {
    view! {
        <svg width=width height=height viewBox="0 0 645 645" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect width="645" height="645" rx="110" fill="url(#paint0_radial_145_21)" />
            <ellipse cx="322.499" cy="321.723" rx="209.042" ry="203.602" fill="#222222" />
            <path
                d="M361.87 291.134C364.518 292.942 366.107 295.89 366.143 299.058L366.898 366.342C366.988 374.408 357.681 379.252 350.94 374.648L219.939 285.182C215.378 282.067 214.296 275.908 217.523 271.427L249.748 226.665C252.974 222.184 259.287 221.075 263.848 224.19L361.87 291.134Z"
                fill="url(#paint1_linear_145_21)"
            />
            <path
                d="M355.946 399.597C358.672 401.497 360.243 404.603 360.137 407.887L358.101 471.14C357.847 479.014 348.729 483.521 342.147 479.026L166.961 359.383C162.4 356.268 161.318 350.11 164.544 345.628L196.714 300.944C199.962 296.432 206.331 295.344 210.892 298.522L355.946 399.597Z"
                fill="url(#paint2_linear_145_21)"
            />
            <path
                d="M454.946 246.014C457.738 247.962 459.312 251.173 459.123 254.535L455.612 316.786C455.174 324.548 446.163 328.874 439.669 324.439L274.103 211.367C269.542 208.252 268.46 202.093 271.687 197.612L303.854 152.931C307.103 148.418 313.475 147.331 318.036 150.512L454.946 246.014Z"
                fill="url(#paint3_linear_145_21)"
            />
            <defs>
                <radialGradient
                    id="paint0_radial_145_21"
                    cx="0"
                    cy="0"
                    r="1"
                    gradientUnits="userSpaceOnUse"
                    gradientTransform="translate(322.5 322.5) rotate(90) scale(322.5)"
                >
                    <stop stop-color="#C8C8C8" />
                    <stop offset="0.338542" stop-color="#CCCCCC" stop-opacity="0.72" />
                    <stop offset="0.619792" stop-color="#CFCFCF" stop-opacity="0.53" />
                    <stop offset="0.78125" stop-color="#D3D3D3" stop-opacity="0.46" />
                    <stop offset="0.864583" stop-color="#CBCBCB" stop-opacity="0.330476" />
                    <stop offset="1" stop-color="#D5D5D5" stop-opacity="0" />
                </radialGradient>
                <linearGradient
                    id="paint1_linear_145_21"
                    x1="254.267"
                    y1="236.292"
                    x2="255.015"
                    y2="338.488"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#636363" />
                    <stop offset="1" stop-color="#ABABAB" />
                </linearGradient>
                <linearGradient
                    id="paint2_linear_145_21"
                    x1="203.43"
                    y1="311.957"
                    x2="192.414"
                    y2="421.055"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#636363" />
                    <stop offset="1" stop-color="#ABABAB" />
                </linearGradient>
                <linearGradient
                    id="paint3_linear_145_21"
                    x1="310.077"
                    y1="163.601"
                    x2="301.549"
                    y2="271.515"
                    gradientUnits="userSpaceOnUse"
                >
                    <stop stop-color="#636363" />
                    <stop offset="1" stop-color="#ABABAB" />
                </linearGradient>
            </defs>
        </svg>
    }
}
