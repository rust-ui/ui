use icons::{Minus, Plus};
use leptos::prelude::*;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};

const DEFAULT_GOAL: i32 = 350;
const GOAL_STEP: i32 = 10;

#[component]
pub fn CardMoveGoal() -> impl IntoView {
    let goal_signal = RwSignal::new(DEFAULT_GOAL);

    let increment = move |_| {
        goal_signal.update(|goal| *goal += GOAL_STEP);
    };

    let decrement = move |_| {
        goal_signal.update(|goal| {
            if *goal >= GOAL_STEP {
                *goal -= GOAL_STEP;
            }
        });
    };

    view! {
        <div class="pt-3 sm:pt-0 sm:pl-2 xl:pl-3">
            <Card>
                <CardHeader class="pb-4">
                    <CardTitle class="text-base">Move Goal</CardTitle>
                    <CardDescription>Set daily activity goal.</CardDescription>
                </CardHeader>
                <CardContent class="pt-0 pb-2">
                    <div class="flex justify-center items-center space-x-2">
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Icon
                            class="rounded-full shrink-0"
                            on:click=decrement
                        >
                            <Minus />
                            <span class="hidden">Decrease</span>
                        </Button>
                        <div class="flex-1 text-center">
                            <div class="text-5xl font-bold tracking-tighter">{goal_signal}</div>
                            <div class="uppercase text-[0.70rem] text-muted-foreground">Calories/day</div>
                        </div>
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Icon
                            class="rounded-full shrink-0"
                            on:click=increment
                        >
                            <Plus />
                            <span class="hidden">Increase</span>
                        </Button>
                    </div>
                    <div class="my-3 h-[60px]">
                        // * See below
                        <ChartMoveGoal />
                    </div>
                </CardContent>
                <CardFooter>
                    <Button class="w-full">Set Goal</Button>
                </CardFooter>
            </Card>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ChartMoveGoal() -> impl IntoView {
    view! {
        <div class="recharts-responsive-container" style="width: 100%; height: 100%; min-width: 0px;">
            <div
                class="recharts-wrapper"
                role="region"
                style="position: relative; cursor: default; width: 100%; height: 100%; max-height: 60px; max-width: 372px;"
            >
                <svg
                    class="opacity-50 recharts-surface fill-secondary"
                    width="372"
                    height="60"
                    viewBox="0 0 372 60"
                    style="width: 100%; height: 100%;"
                >
                    <title></title>
                    <desc></desc>
                    <defs>
                        <clipPath id="recharts18-clip">
                            <rect x="5" y="5" height="50" width="362"></rect>
                        </clipPath>
                    </defs>
                    <g class="recharts-layer recharts-bar">
                        <g class="recharts-layer recharts-bar-rectangles">
                            <g class="recharts-layer">
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="7.7846153846153845"
                                        y="5"
                                        width="22"
                                        height="50"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 7.7846153846153845,5 h 22 v 50 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="35.63076923076923"
                                        y="17.5"
                                        width="22"
                                        height="37.5"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 35.63076923076923,17.5 h 22 v 37.5 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="63.47692307692308"
                                        y="30"
                                        width="22"
                                        height="25"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 63.47692307692308,30 h 22 v 25 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="91.32307692307693"
                                        y="17.5"
                                        width="22"
                                        height="37.5"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 91.32307692307693,17.5 h 22 v 37.5 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="119.16923076923077"
                                        y="30"
                                        width="22"
                                        height="25"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 119.16923076923077,30 h 22 v 25 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="147.01538461538462"
                                        y="20.25"
                                        width="22"
                                        height="34.75"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 147.01538461538462,20.25 h 22 v 34.75 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="174.8615384615385"
                                        y="31.375000000000004"
                                        width="22"
                                        height="23.624999999999996"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 174.8615384615385,31.375000000000004 h 22 v 23.624999999999996 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="202.70769230769233"
                                        y="25.125"
                                        width="22"
                                        height="29.875"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 202.70769230769233,25.125 h 22 v 29.875 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="230.55384615384617"
                                        y="17.5"
                                        width="22"
                                        height="37.5"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 230.55384615384617,17.5 h 22 v 37.5 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="258.4"
                                        y="30"
                                        width="22"
                                        height="25"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 258.4,30 h 22 v 25 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="286.24615384615385"
                                        y="20.25"
                                        width="22"
                                        height="34.75"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 286.24615384615385,20.25 h 22 v 34.75 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="314.0923076923077"
                                        y="31.375000000000004"
                                        width="22"
                                        height="23.624999999999996"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 314.0923076923077,31.375000000000004 h 22 v 23.624999999999996 h -22 Z"
                                    ></path>
                                </g>
                                <g class="recharts-layer recharts-bar-rectangle">
                                    <path
                                        x="341.9384615384616"
                                        y="11.374999999999998"
                                        width="22"
                                        height="43.625"
                                        radius="0"
                                        class="recharts-rectangle"
                                        d="M 341.9384615384616,11.374999999999998 h 22 v 43.625 h -22 Z"
                                    ></path>
                                </g>
                            </g>
                        </g>
                        <g class="recharts-layer"></g>
                    </g>
                </svg>
            </div>
        </div>
    }
}
