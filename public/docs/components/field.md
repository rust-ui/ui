+++
title = "Field"
description = "Rust/UI components for composing accessible form fields with labels, descriptions, and error messages."
tags = ["input"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticField />



## Installation

<StaticInstallField />



## Usage

```rust
use crate::components::ui::field::{
    Field,
    FieldContent,
    FieldDescription,
    FieldError,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSeparator,
    FieldSet,
    FieldTitle,
    FieldVariant,
};
```

```rust
<FieldSet>
    <FieldLegend>Profile</FieldLegend>
    <FieldDescription>This appears on invoices and emails.</FieldDescription>
    <FieldGroup>
        <Field>
            <FieldLabel html_for="name">Full name</FieldLabel>
            <Input attr:id="name" placeholder="Evil Rabbit" />
            <FieldDescription>This appears on invoices and emails.</FieldDescription>
        </Field>
        <Field>
            <FieldLabel html_for="username">Username</FieldLabel>
            <Input attr:id="username" attr:aria-invalid="true" />
            <FieldError>"Choose another username."</FieldError>
        </Field>
        <Field variant=FieldVariant::Horizontal>
            <Switch />
            <FieldLabel>"Subscribe to the newsletter"</FieldLabel>
        </Field>
    </FieldGroup>
</FieldSet>
```



## Examples

### Input

Field wrapping an [Input](/docs/components/input) with label and description. Demonstrates vertical orientation with description above or below the control.

<StaticFieldInput />

### Fieldset

Semantic grouping of related fields using `FieldSet` and `FieldLegend` for accessible form sections.

<StaticFieldFieldset />

### Checkbox

Horizontal field orientation with [Checkbox](/docs/components/checkbox) controls and `FieldContent` for stacked label and description.

<StaticFieldCheckbox />


### Input Inline

Simple inline search form using `Field` horizontal orientation — no [InputGroup](/docs/components/input_group) needed for basic input + button rows.

<StaticFieldInputInline />


### Switch Choice Card

Wrap a `FieldLabel` around a `Field` + `FieldContent` + [Switch](/docs/components/switch) to create a settings card that highlights when toggled. Clicking anywhere on the card toggles the switch.

<StaticSwitchChoiceCard />


### Radio Choice Card

Wrap `FieldLabel` around `Field` + `FieldContent` + `RadioGroupItem` inside a `RadioGroup` for a plan/tier selector where the entire card row is clickable.

<StaticRadioChoiceCard />



### RTL

Payment form built with Field components and Arabic labels. Grid layout, label alignment, and logical spacing all adapt to RTL automatically.

<StaticFieldRtl />

## See Also

- [Form](/docs/components/form)
- [Input](/docs/components/input)
- [Checkbox](/docs/components/checkbox)
- [Switch](/docs/components/switch)
- [Label](/docs/components/label)
- [Separator](/docs/components/separator)
