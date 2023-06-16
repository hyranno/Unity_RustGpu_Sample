Shader "Unlit/SampleShader"
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
                #include "/Assets/HLSL.symlink/shader.main_vs.hlsl"
            #endif
            #ifdef SHADER_STAGE_FRAGMENT
                #include "/Assets/HLSL.symlink/shader.main_fs.hlsl"
            #endif

            ENDHLSL
        }
    }
}
