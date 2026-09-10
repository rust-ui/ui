pub const TEMPLATE_STYLES: &str = r#"
:root {
    --leptoaster-width: 320px;
    --leptoaster-max-width: 80vw;
    --leptoaster-z-index: 9999;

    --leptoaster-font-family: Arial;
    --leptoaster-font-size: 14px;
    --leptoaster-line-height: 20px;
    --leptoaster-font-weight: 600;

    --leptoaster-progress-height: 4px;

    --leptoaster-info-background-color: #f0f9ff;
    --leptoaster-info-border-color: #bae6fd;
    --leptoaster-info-text-color: #0369a1;

    --leptoaster-success-background-color: #f0fdf4;
    --leptoaster-success-border-color: #bbf7d0;
    --leptoaster-success-text-color: #15803d;

    --leptoaster-warn-background-color: #fefce8;
    --leptoaster-warn-border-color: #fde68a;
    --leptoaster-warn-text-color: #b45309;

    --leptoaster-error-background-color: #fef2f2;
    --leptoaster-error-border-color: #fecaca;
    --leptoaster-error-text-color: #dc2626;
}

@keyframes leptoaster-slide-in-right {
    from {
        transform: translateX(calc(var(--leptoaster-width) + 12px * 2));
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

@keyframes leptoaster-slide-out-right {
    from {
        transform: translateX(0);
        opacity: 1;
    }
    to {
        transform: translateX(calc(var(--leptoaster-width) + 12px * 2));
        opacity: 0;
    }
}

@keyframes leptoaster-slide-in-left {
    from {
        transform: translateX(calc(-1 * (var(--leptoaster-width) + 12px * 2)));
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

@keyframes leptoaster-slide-out-left {
    from {
        transform: translateX(0);
        opacity: 1;
    }
    to {
        transform: translateX(calc(-1 * (var(--leptoaster-width) + 12px * 2)));
        opacity: 0;
    }
}

.leptoaster-stack-container-bottom {
    display: flex;
    flex-direction: column-reverse;
    align-items: flex-end;
    width: var(--leptoaster-width);
    max-width: var(--leptoaster-max-width);
}

.leptoaster-stack-container-top {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    width: var(--leptoaster-width);
    max-width: var(--leptoaster-max-width);
}
"#;
