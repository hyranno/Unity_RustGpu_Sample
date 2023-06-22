# UnityでRustGPU製のシェーダを使いたかった話
Unityで使われてるHLSLだと高レベル言語機能が足りてなくてつらい。
ので[RustGPU](https://github.com/EmbarkStudios/rust-gpu)を使ってRustでシェーダを書いてSPIR-Vに出力し、
それを[SPIRV-Cross](https://github.com/KhronosGroup/SPIRV-Cross)と[そのラッパー](https://github.com/grovesNL/spirv_cross)でHLSLに変換して
Unityに取り込みたかった。が、つらそうなので中止。

## 構成
Unityプロジェクト内にshaderクレートを置き、その下にshader_srcクレートを置いた。shaderクレートはcargo runするとシェーダをビルドしてHLSLに変換して吐き出す。HLSLを吐き出すディレクトリに対してシンボリックリンクを張り、これを通してUnityからHLSLが見えるようにした。

## 何がつらいか
* Unityではシェーダのクロスプラットフォーム対応にHLSLのプリプロセッサを使っている。RustGPUからSPIRV-Cross経由で吐いたHLSLは当然これらを使っていないので、Unityのクロスプラットフォーム対応を捨てることになる。
* SPIRV-CrossではHLSLの[geometry](https://github.com/KhronosGroup/SPIRV-Cross/issues/904), [hull](https://github.com/KhronosGroup/SPIRV-Cross/issues/905), [domain](https://github.com/KhronosGroup/SPIRV-Cross/issues/906)ステージに対応していない。難しい問題を幾つか孕んでいる[らしい](https://github.com/KhronosGroup/SPIRV-Cross/pull/1739#issuecomment-926705870)。

## どこでやめたか
下記の問題に当たった時に、上記のつらみを抱えたままやり通すほどのうまみはないかと感じて終了。
* Unityからシェーダにデータを渡すcbufferをRustGPU側でどう書くかが分からない。間にSPIRV-Crossを挟むので対応関係が分かりづらい。
* RustGPUではmatrixの対応がまだ甘いらしく、glam(標準で使われるベクトル計算用クレート)のMat4等がSPIR-VのOpTypeMatrixに対応してなかったり、MatrixStrideデコレータが付かなかったりする。これは実装してpull requestを出せる程度かもなとは思うが。

## その他
* RustのSPIRV-CrossラッパーはSPIRV-Crossのフル機能をカバーしているわけではないので、適宜機能を追加する必要がある。今回はvertex_attribute_remapがそれで、[pull request](https://github.com/grovesNL/spirv_cross/pull/181)を出してる人が居たのでそちらを使わせてもらっている。
* SPIR-VやHLSLは1つのファイルでエントリポイントを複数含むことができるが、SPIRV-Crossは[対応していない](https://github.com/KhronosGroup/SPIRV-Cross/issues/317)。今回はUnity側のプリプロセッサでステージごとに読むファイルを切り替えて対応。

## 他のゲームエンジンについて
* UnrealEngineはHLSLなので同様のつらみが予想される。
* Rust製のゲームエンジンでは[Fyrox](https://fyrox.rs/)がGLSL、[Bevy](https://bevyengine.org/)がWGSLとGLSLに対応している様子。WGSLへの変換は[Tint](https://dawn.googlesource.com/tint)を使う事になりそう。

