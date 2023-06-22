Shader "Unlit/WhatIWant"
{
    SubShader
    {
        Tags { "RenderType"="Opaque" }

        Pass
        {
            HLSLPROGRAM
            #pragma vertex main
            #pragma fragment main

            #ifdef SHADER_STAGE_VERTEX


cbuffer UnityPerFrame
{
    float4x4 unity_MatrixVP;
};

cbuffer UnityPerDraw
{
    float4x4 unity_ObjectToWorld;
};

static float4 gl_Position;
static float4 in_position;

struct SPIRV_Cross_Input
{
    float4 in_position : POSITION;
};

struct SPIRV_Cross_Output
{
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    gl_Position = mul(unity_MatrixVP, mul(unity_ObjectToWorld, in_position));
}

SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
{
    in_position = stage_input.in_position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    return stage_output;
}
            #endif
            #ifdef SHADER_STAGE_FRAGMENT

static float4 _output;

struct SPIRV_Cross_Output
{
    float4 _output : SV_Target0;
};

void frag_main()
{
    _output = float4(1.0f, 0.0f, 0.0f, 1.0f);
}

SPIRV_Cross_Output main()
{
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output._output = _output;
    return stage_output;
}

            #endif
            ENDHLSL

        }
    }
}
