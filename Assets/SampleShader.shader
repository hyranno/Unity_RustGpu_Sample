Shader "Unlit/SampleShader"
{
    SubShader
    {
        Tags { "RenderType"="Opaque" }

        Pass
        {
            HLSLPROGRAM
            #pragma vertex main_vs
            #pragma fragment main_fs

            #include "/Assets/HLSL.symlink/shader.main_vs.hlsl"
            #include "/Assets/HLSL.symlink/shader.main_fs.hlsl"

            ENDHLSL
        }
    }
}
