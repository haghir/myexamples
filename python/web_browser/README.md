# Web Browser Example in Python

This project demonstrates how to create a Python program that behaves like a web browser, including:

1. **Accessing web pages** - Downloading HTML content from URLs
2. **Parsing HTML** - Converting HTML into a structured format
3. **Generating DOM tree** - Creating a tree representation of the document structure
4. **Running JavaScript** - Executing JavaScript code using a headless browser

## Features

- `WebBrowser` class that mimics browser behavior
- `DOMNode` class for representing DOM tree nodes
- HTML parsing using BeautifulSoup
- JavaScript execution using Playwright
- Support for both static and dynamic (JavaScript-rendered) pages

## Installation

1. Install Python dependencies:
```bash
pip install -r requirements.txt
```

2. Install Playwright browsers:
```bash
playwright install chromium
```

## Usage

### Basic Example

```python
from web_browser_example import WebBrowser

browser = WebBrowser()

# Complete workflow
html, dom, rendered = browser.render_page_with_js("https://example.com")

# Print DOM tree
browser.print_dom_tree()
```

### Step-by-Step Usage

```python
from web_browser_example import WebBrowser

browser = WebBrowser()

# 1. Download a web page
html = browser.download_page("https://example.com")

# 2. Parse HTML to DOM tree
dom = browser.parse_html_to_dom(html)

# 3. Run JavaScript (gets rendered HTML after JS execution)
rendered = browser.run_javascript(url="https://example.com")
```

### Execute Custom JavaScript

```python
browser = WebBrowser()

js_code = """
() => {
    return {
        title: document.title,
        url: window.location.href,
        headings: Array.from(document.querySelectorAll('h1')).map(h => h.textContent)
    };
}
"""

result = browser.run_javascript(js_code, "https://example.com")
print(result)
```

## Running the Example

```bash
python web_browser_example.py
```

This will run three examples:
1. Simple HTML page parsing
2. Custom JavaScript execution
3. Dynamic page with JavaScript rendering

## Architecture

### DOMNode Class
Represents a node in the DOM tree with:
- Tag name
- Text content
- Attributes
- Children nodes
- Parent reference

### WebBrowser Class
Main browser implementation with methods:
- `download_page(url)` - Downloads HTML content
- `parse_html_to_dom(html)` - Converts HTML to DOM tree
- `run_javascript(js_code, url)` - Executes JavaScript using Playwright
- `render_page_with_js(url)` - Complete workflow

## Notes

- Playwright requires browser binaries to be installed separately
- JavaScript execution uses Chromium in headless mode
- The DOM tree representation is simplified but functional
- For production use, consider using more robust libraries like Selenium or Playwright directly

