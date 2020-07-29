#version 300 es
// Basic shader, same as shadertoys default shader

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;

uniform vec2 iResolution;
uniform float iTime;



void mainImage( out vec4 fragColor, in vec2 fragCoord )
{   
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}


void main() {
       mainImage(FragColor, screen_pos.xy * iResolution);
}
