#include <GLFW/glfw3.h>

// Initialize GLFW and create a window
void initWindow() {
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    GLFWwindow* window = glfwCreateWindow(640, 480, "Triangle Rendering", NULL, NULL);
    glfwMakeContextCurrent(window);
}

// Load and compile shaders
void loadShaders() {
    // Vertex shader
    const char* vertexShader = R"(
        #version 330 core
        in vec3 pos;
        void main() {
            gl_Position = vec4(pos, 1.0);
        }
    )";

    // Fragment shader
    const char* fragmentShader = R"(
        #version 330 core
        out vec4 fragColor;
        void main() {
            fragColor = vec4(1.0, 0.0, 0.0, 1.0); // Red color
        }
    )";

    // Compile shaders
    GLuint vertexShaderId = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShaderId, 1, &vertexShader, NULL);
    glCompileShader(vertexShaderId);

    GLuint fragmentShaderId = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShaderId, 1, &fragmentShader, NULL);
    glCompileShader(fragmentShaderId);

    // Create a program and attach shaders
    GLuint programId = glCreateProgram();
    glAttachShader(programId, vertexShaderId);
    glAttachShader(programId, fragmentShaderId);
    glLinkProgram(programId);
}

// Render a triangle
void renderTriangle() {
    // Clear the screen
    glClearColor(0.0f, 0.0f, 0.4f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);

    // Use the program
    glUseProgram(programId);

    // Define vertex data
    GLfloat vertices[] = {
        -0.5f, -0.5f, 0.0f,
         0.5f, -0.5f, 0.0f,
         0.0f,  0.5f, 0.0f
    };

    // Vertex Buffer Object (VBO)
    GLuint vboId;
    glGenBuffers(1, &vboId);
    glBindBuffer(GL_ARRAY_BUFFER, vboId);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    // Draw the triangle
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
    glEnableVertexAttribArray(0);
    glDrawArrays(GL_TRIANGLES, 0, 3);
}

int main() {
    // Initialize GLFW and create a window
    initWindow();

    // Load shaders
    loadShaders();

    // Main game loop
    while (!glfwWindowShouldClose(window)) {
        // Render the triangle
        renderTriangle();

        // Swap buffers and poll for events
        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    // Clean up
    glDeleteProgram(programId);
    glDeleteBuffers(1, &vboId);
    glfwTerminate();
    return 0;
}