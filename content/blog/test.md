---
title: "finishing grade 12 comp sci in a weekend"
description: "how i finished grade 12 comp sci in a weekend, with no prior java experience."
imageURL: "/images/posts/cs-in-a-weekend/ems-unselected.png"
imageAlt: "screenshot of an employee management system window with a table of employees with none selected. add, update, delete, and view employee buttons are on the right sidebar, with the last 3 grayed out."
---


```rust
pub fn document(path: Option<&str>, content: Node) -> Node {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {seo(path)}
                <link rel="stylesheet" href="/public/css/styles.css" />
            </head>

            <body class="bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100">
                {layout(content)}
            </body>
        </html>
    }

    btml! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {seo(path)}
                <link rel="stylesheet" href="/public/css/styles.css" />
            </head>

            <body class="bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100">
                {layout(content)}
            </body>
        </html>
    }
}
```

```html
<!DOCTYPE html>
<html lang="en">
    <head>
        {seo(path)}
        <link rel="stylesheet" href="/public/css/styles.css" />
    </head>

    <body class="bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100">
        {layout(content)}
    </body>
</html>
```
