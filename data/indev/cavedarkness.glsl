uniform mat4 rg3d_worldMatrix;

void main
(
    // A set of properties, there could be any amount of properties.
    properties: [
        (
            // Each property must have a name. This name must match with respective
            // uniforms! That's is the whole point of having properties.
            name: "diffuseTexture",
            // Value has limited set of possible variants.
            value: Sampler(default: None, fallback: White)
        )
    ],
    // A set of render passes (see next section for more info)
    passes: [
        (
            // Name must match with the name of either standard render pass (see below) or
            // one of your passes.
            name: "Forward",
            // A set of parameters that regulate renderer pipeline state.
            // This is mandatory field of each render pass.
            draw_parameters: DrawParameters(
                // A face to cull. Either Front or Back.
                cull_face: Some(Back),
                // Color mask. Defines which colors should be written to render target.
                color_write: ColorMask(
                    red: true,
                    green: true,
                    blue: true,
                    alpha: true,
                ),
                // Whether to modify depth buffer or not.
                depth_write: true,
                // Whether to use stencil test or not.
                stencil_test: None,
                // Whether to perform depth test when drawing.
                depth_test: true,
                // Blending options.
                blend: Some(BlendFunc(
                    sfactor: SrcAlpha,
                    dfactor: OneMinusSrcAlpha,
                )),
                // Stencil options.
                stencil_op: StencilOp(
                    fail: Keep,
                    zfail: Keep,
                    zpass: Keep,
                    wr  ite_mask: 0xFFFF_FFFF,
                ),
            ),
            // Vertex shader code.
            vertex_shader:
                r#"
                layout(location = 0) in vec3 vertexPosition;
                layout(location = 1) in vec2 vertexTexCoord;
                uniform mat4 rg3d_worldViewProjection;
                out vec2 texCoord;
                void main()
                {
                    texCoord = vertexTexCoord;
                    gl_Position = rg3d_worldViewProjection * vertexPosition;
                }
                "#;
            // Pixel shader code.
            pixel_shader:
                r#"
                // Note that the name of this uniform match the name of the property up above.
                uniform sampler2D diffuseTexture;
                out vec4 FragColor;
                in vec2 texCoord;
                void main()
                {
                    FragColor = diffuseColor * texture(diffuseTexture, texCoord);
                }
                "#;
        )
    ],
)
