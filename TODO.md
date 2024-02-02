# Interface

When opened, shows a list of sessions, or if none, drops into a new session immediately. This shows a title bar with the page title, and page content on the rest of the screen.

Links to wikipedia pages are shown in blue with `[<key>]` behind them, hit `<key>` to navigate to that page.

To view a next page, press space (shift space to go back). Scroll through the page with up / down arrow and page up / page down.

## Implementation stuff

Parse the HTML, get all the `<p>` tags and only display those (will miss some information, but good enough for now). Wikipedia links are treated specially: `<a rel="mw:WikiLink" href="./West_Philadelphia"` in the HTML that Wikipedia serves.

# Sessions

Sessions track interest in categories by monitoring all user input:

-   Time spent on page.
-   Percentage of page viewed.
-   Links clicked on page

The function that determines where and how to get a next page uses these metrics.

# New pages

Sorted from coarse grained to fine grained:

-   The `/random/` endpoint
-   // engineer some way to get a similar page in the same category, or one 'category' up, think of using the list of lists of lists as indexer / jumpoff point, or find a way into the categories.
-   The `/related/` endpoint
-   A user clicking on a page
