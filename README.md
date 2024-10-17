Here is the markdown documentation for the provided Rust code using GLFW to move a triangle around on the screen with arrow keys:

```markdown
# Rustangles: Moving a Triangle with Arrow Keys

## Overview

**Rustangles** is a simple OpenGL application written in Rust using the `glfw` and `gl` crates. It displays a triangle on the screen that can be moved using the arrow keys (`Up`, `Down`, `Left`, `Right`) or `W`, `A`, `S`, `D`. The application sets up an OpenGL context and uses shaders to render the triangle.

## Prerequisites

- **Rust** (nightly recommended)
- **GLFW** library installed on your system.
- The following Rust crates:
  - `glfw`
  - `gl`
  - `std::convert::TryInto`

### Installation

1. Add the required dependencies to your `Cargo.toml`:

   ```toml
   [dependencies]
   glfw = "0.42.0"
   gl = "0.14.0"
   ```

2. Ensure you have the `GLFW` library installed. For Linux, use:
   ```bash
   sudo apt-get install libglfw3-dev
   ```

3. Clone the repository and build the project:

   ```bash
   git clone https://github.com/yourusername/rustangles.git
   cd rustangles
   cargo run
   ```

## How It Works

1. **OpenGL Context Setup**: 
   - Initializes a window using GLFW with a 3.3 OpenGL context.
   - Sets up the window to be non-resizable.

2. **Shaders**:
   - **Vertex Shader**: Controls the position of the vertices.
   - **Fragment Shader**: Sets the color of the triangle.

3. **Triangle Drawing**:
   - Vertices are defined in a `vertex_into_frame` function, which sets up the vertex data and buffers.
   - Uses a Vertex Array Object (VAO) and Vertex Buffer Object (VBO) for drawing the triangle.

4. **Input Handling**:
   - Listens for key presses (arrow keys or `W`, `A`, `S`, `D`) and moves the triangle up, down, left, or right.
   - Adjusts the `x` or `y` values of the vertices and updates the VAO accordingly.

5. **Rendering Loop**:
   - Clears the screen to a background color.
   - Uses the shaders to draw the triangle.
   - Swaps buffers and polls for events to keep the window responsive.

## Code Structure

- `main()`: Entry point. Sets up GLFW, OpenGL context, and shaders, then enters the rendering loop.
- `clear_color()`: Sets the background color for the window.
- `vertex_into_frame()`: Configures vertex data and returns a VAO for drawing the triangle.
- `gl_get_string()`: Helper function to retrieve OpenGL version strings.

## Controls

| Key       | Action                  |
|-----------|-------------------------|
| `Up`/`W`  | Move triangle up        |
| `Down`/`S`| Move triangle down      |
| `Left`/`A`| Move triangle left      |
| `Right`/`D`| Move triangle right    |
| `Escape`  | Close the window        |

## Example

Once the application is running, use the arrow keys to move the triangle around the screen. The triangle will respond to continuous key presses, allowing for smooth movement.

## Troubleshooting

- **GLFW Initialization Error**: Ensure that `libglfw3` is installed and properly linked.
- **Shader Compilation Errors**: Verify the syntax of the shaders in `VERT_SHADER` and `FRAG_SHADER`.
- **Window Not Displaying**: Check if your GPU supports OpenGL 3.3 or update your GPU drivers.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
