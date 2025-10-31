"""
A Python example that behaves like a web browser:
- Accesses a web page
- Downloads it
- Parses HTML and generates DOM tree
- Runs JavaScript
"""

import requests
from bs4 import BeautifulSoup
from playwright.sync_api import sync_playwright
from typing import Optional, Dict, List
import json


class DOMNode:
    """Represents a node in the DOM tree"""
    
    def __init__(self, tag: Optional[str] = None, text: str = "", attributes: Optional[Dict] = None):
        self.tag = tag
        self.text = text
        self.attributes = attributes or {}
        self.children: List['DOMNode'] = []
        self.parent: Optional['DOMNode'] = None
    
    def add_child(self, child: 'DOMNode'):
        """Add a child node to this node"""
        child.parent = self
        self.children.append(child)
    
    def to_dict(self) -> dict:
        """Convert the DOM node to a dictionary representation"""
        result = {
            'tag': self.tag,
            'text': self.text.strip() if self.text else '',
            'attributes': self.attributes
        }
        if self.children:
            result['children'] = [child.to_dict() for child in self.children]
        return result
    
    def __repr__(self) -> str:
        if self.tag:
            attrs = ' '.join([f'{k}="{v}"' for k, v in self.attributes.items()])
            attrs_str = f' {attrs}' if attrs else ''
            return f'<{self.tag}{attrs_str}>'
        return f'Text: {self.text[:50]}...' if len(self.text) > 50 else f'Text: {self.text}'


class WebBrowser:
    """A simple web browser implementation"""
    
    def __init__(self):
        self.html_content = None
        self.dom_tree: Optional[DOMNode] = None
        self.url = None
    
    def download_page(self, url: str) -> str:
        """Download the web page content"""
        print(f"🌐 Accessing: {url}")
        headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }
        response = requests.get(url, headers=headers, timeout=10)
        response.raise_for_status()
        self.html_content = response.text
        self.url = url
        print(f"✅ Downloaded {len(self.html_content)} bytes")
        return self.html_content
    
    def parse_html_to_dom(self, html: Optional[str] = None) -> DOMNode:
        """Parse HTML and generate a DOM tree"""
        if html is None:
            html = self.html_content
        
        if html is None:
            raise ValueError("No HTML content available. Download a page first.")
        
        print("📄 Parsing HTML...")
        soup = BeautifulSoup(html, 'html.parser')
        
        # Create root node
        root = DOMNode(tag='document', text='')
        
        # Helper function to convert BeautifulSoup elements to DOM nodes
        def soup_to_dom(soup_element, parent_dom_node: DOMNode):
            if soup_element.name is None:  # Text node
                text = str(soup_element).strip()
                if text:
                    text_node = DOMNode(tag=None, text=text)
                    parent_dom_node.add_child(text_node)
            else:  # Element node
                # Create DOM node for this element
                dom_node = DOMNode(
                    tag=soup_element.name,
                    attributes=dict(soup_element.attrs) if soup_element.attrs else {}
                )
                parent_dom_node.add_child(dom_node)
                
                # Process children
                for child in soup_element.children:
                    soup_to_dom(child, dom_node)
        
        # Build DOM tree
        soup_to_dom(soup, root)
        
        self.dom_tree = root
        print(f"✅ DOM tree generated with {self._count_nodes(root)} nodes")
        return root
    
    def _count_nodes(self, node: DOMNode) -> int:
        """Count total nodes in the DOM tree"""
        count = 1
        for child in node.children:
            count += self._count_nodes(child)
        return count
    
    def run_javascript(self, js_code: Optional[str] = None, url: Optional[str] = None) -> str:
        """
        Run JavaScript code using Playwright (headless browser)
        If js_code is None, it will execute the page's own JavaScript
        """
        if url is None:
            url = self.url
        
        if url is None:
            raise ValueError("No URL available. Provide a URL or download a page first.")
        
        print("🚀 Running JavaScript with Playwright...")
        
        with sync_playwright() as p:
            # Launch browser
            browser = p.chromium.launch(headless=True)
            page = browser.new_page()
            
            # Navigate to the page
            page.goto(url, wait_until='networkidle')
            
            # Wait for JavaScript to execute
            page.wait_for_load_state('domcontentloaded')
            
            # If custom JavaScript code provided, execute it
            if js_code:
                result = page.evaluate(js_code)
                print(f"✅ JavaScript executed")
                return json.dumps(result, indent=2) if result else "JavaScript executed successfully"
            
            # Otherwise, get the rendered HTML (after JavaScript execution)
            rendered_html = page.content()
            
            # Get page title
            title = page.title()
            
            # Get some basic info
            body_text = page.evaluate('document.body.innerText')
            
            browser.close()
            
            print(f"✅ JavaScript execution complete")
            print(f"   Title: {title}")
            print(f"   Body length: {len(body_text)} characters")
            
            return rendered_html
    
    def render_page_with_js(self, url: str) -> tuple:
        """
        Complete workflow: download, parse, and run JavaScript
        Returns (html_content, dom_tree, rendered_html)
        """
        # Download
        html = self.download_page(url)
        
        # Parse to DOM
        dom = self.parse_html_to_dom(html)
        
        # Run JavaScript (this will get the rendered HTML)
        rendered_html = self.run_javascript(url=url)
        
        return html, dom, rendered_html
    
    def print_dom_tree(self, node: Optional[DOMNode] = None, indent: int = 0):
        """Print the DOM tree in a readable format"""
        if node is None:
            node = self.dom_tree
        
        if node is None:
            print("No DOM tree available")
            return
        
        prefix = "  " * indent
        if node.tag:
            attrs = ' '.join([f'{k}="{v}"' for k, v in node.attributes.items()])
            attrs_str = f' {attrs}' if attrs else ''
            print(f"{prefix}<{node.tag}{attrs_str}>")
        elif node.text:
            text = node.text.strip()
            if text:
                print(f"{prefix}Text: {text[:100]}")
        
        for child in node.children:
            self.print_dom_tree(child, indent + 1)
        
        if node.tag:
            print(f"{prefix}</{node.tag}>")


def main():
    """Example usage"""
    browser = WebBrowser()
    
    # Example 1: Simple static HTML page
    print("=" * 60)
    print("Example 1: Simple HTML page")
    print("=" * 60)
    
    try:
        # Use a simple test page
        url = "https://example.com"
        html, dom, rendered = browser.render_page_with_js(url)
        
        print("\n--- Original HTML (first 500 chars) ---")
        print(html[:500])
        
        print("\n--- DOM Tree (first few levels) ---")
        browser.print_dom_tree(dom.children[0] if dom.children else None)
        
        print("\n--- DOM Tree as JSON (first node) ---")
        if dom.children:
            print(json.dumps(dom.children[0].to_dict(), indent=2)[:500])
        
    except Exception as e:
        print(f"Error: {e}")
    
    # Example 2: Execute custom JavaScript
    print("\n" + "=" * 60)
    print("Example 2: Execute custom JavaScript")
    print("=" * 60)
    
    try:
        url = "https://example.com"
        js_code = """
        () => {
            return {
                title: document.title,
                url: window.location.href,
                headings: Array.from(document.querySelectorAll('h1, h2, h3')).map(h => h.textContent),
                links: Array.from(document.querySelectorAll('a')).map(a => a.href).slice(0, 5)
            };
        }
        """
        result = browser.run_javascript(js_code, url)
        print("\n--- JavaScript Execution Result ---")
        print(result)
        
    except Exception as e:
        print(f"Error: {e}")
    
    # Example 3: Page with JavaScript (dynamic content)
    print("\n" + "=" * 60)
    print("Example 3: Page with JavaScript rendering")
    print("=" * 60)
    
    try:
        # This page uses JavaScript to render content
        url = "https://quotes.toscrape.com/js/"
        html, dom, rendered = browser.render_page_with_js(url)
        
        print(f"\nOriginal HTML length: {len(html)}")
        print(f"Rendered HTML length: {len(rendered)}")
        print(f"Difference: {len(rendered) - len(html)} bytes (JavaScript-added content)")
        
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()

