# 🌟 Welcome to Random Markdown

> "Markdown is not a replacement for HTML, but it is a great tool for writers!"  
> — *Someone Wise*



## 🚀 Features

- **Bold** and *Italic* text
- `Inline code` and syntax highlighting
- [Links](https://www.markdownguide.org)
- Images:  
  ![Random Cat](https://placekitten.com/200/300)

---

### 📋 To-Do List

- [x] Learn Markdown
- [ ] Master GitHub README files
- [ ] Build a static site with Markdown content

---

## 💻 Code Block

```python
def hello_markdown():
    print("Hello, Markdown World!")
```


## Usage

```rust
use crate::components::ui::card::{
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
};
```

```rust
<Card>
    <CardHeader>
        <CardTitle>"Card Title"</CardTitle>
        <CardDescription>"Card Description"</CardDescription>
    </CardHeader>
    <CardContent>
        <p>"Card Content"</p>
    </CardContent>
    <CardFooter>
        <p>"Card Footer"</p>
    </CardFooter>
</Card>
```



## 🦀 Rust Examples

### Basic Function
```rust
fn main() {
    println!("Hello, Rust!");
}
```

### Data Structures
```rust
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: Option<String>,
}

impl User {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            email: None,
        }
    }
}
```

### Error Handling
```rust
use std::fs;
use std::io::Result;

fn read_file_contents(path: &str) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.trim().to_string())
}
```

### Pattern Matching
```rust
fn process_number(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1..=10 => format!("small number: {}", n),
        11..=100 => format!("medium number: {}", n),
        _ => "large number".to_string(),
    }
}
```

### Traits and Generics
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

fn render<T: Drawable>(item: &T) {
    item.draw();
}
```

