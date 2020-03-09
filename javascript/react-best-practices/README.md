## Styling

- ### [styled-components](https://www.npmjs.com/package/styled-components)
  I've found `styled-components` to be a much more effective method of styling than class names and separate stylesheets.

## Organisation

Markup should be divided into:

- Components - Smallest unit of React code. Very small reusable pieces of UI.
- Containers - Smallest unit of feature ui. These will be composed of multiple components and will represent a complete feature or area of the app, e.g. a form would be a Container, an input field within that form would be a Component.
- Layouts - Components that are concerned solely with layout. The components will receive children as props which are then slotted into predetermined places.
- Pages - Composes Layouts and Containers. Shared state will live here and hooks for passing around higher level state (i.e. between pages) will be used here.
