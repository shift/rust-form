# UI Kit Component

A comprehensive UI component library for Rust-form applications.

## Components

### Button
A styled button component with multiple variants.

**Usage:**
```tera
{% include component_template(component="ui-kit", template="button.tera") %}
  text="Click me"
  variant="primary"
  type="button"
{% end %}
```

**Parameters:**
- `text`: Button text (default: "Button")
- `variant`: Button style - "primary", "secondary" (default: "primary")
- `type`: Button type - "button", "submit", "reset" (default: "button")
- `class`: Additional CSS classes
- `disabled`: Boolean to disable the button
- `id`: Button ID attribute

### Input
A form input field with label, validation, and help text.

**Usage:**
```tera
{% include component_template(component="ui-kit", template="input.tera") %}
  name="email"
  type="email"
  label="Email Address"
  placeholder="Enter your email"
  required=true
{% end %}
```

**Parameters:**
- `name`: Input name attribute (required)
- `type`: Input type (default: "text")
- `label`: Field label
- `placeholder`: Placeholder text
- `value`: Default value
- `required`: Boolean for required field
- `disabled`: Boolean to disable input
- `error`: Error message to display
- `help`: Help text to display
- `class`: Additional CSS classes
- `id`: Input ID (defaults to name)

### Form
A form wrapper with styling and optional submit button.

**Usage:**
```tera
{% include component_template(component="ui-kit", template="form.tera") %}
  action="/submit"
  method="POST"
  submit_text="Save"
  content="{{ form_fields }}"
{% end %}
```

**Parameters:**
- `action`: Form action URL
- `method`: HTTP method (default: "POST")
- `submit_text`: Submit button text (optional)
- `content`: Form content
- `class`: Additional CSS classes
- `id`: Form ID

### Card
A content card with header, body, and footer sections.

**Usage:**
```tera
{% include component_template(component="ui-kit", template="card.tera") %}
  title="User Profile"
  header=true
  content="{{ user_info }}"
  footer="{{ save_button }}"
{% end %}
```

**Parameters:**
- `title`: Card title
- `header`: Boolean to show header
- `content`: Main card content
- `footer`: Footer content
- `header_actions`: Action buttons in header
- `class`: Additional CSS classes

## Installation

Add to your `rustform.yml`:

```yaml
components:
  ui-kit: "path:./examples/components/ui-kit"
```

## License

MIT