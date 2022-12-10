# console_tetris_rs

## Rustの基本

| No | 用語 | 意味 |
| --- | --- | --- |
| (1) | パッケージ | クレートのビルド・テスト・共有を可能にするためのCargoの単位のこと。 |
| (2) | Cargo.toml | パッケージのメタデータや、パッケージをビルドするために必要な外部クレートへの依存関係を記述する。パッケージのマニフェストと呼ばれる。 |
| (3) | クレート | パッケージに1つのまとまりの機能を提供する単位で、ルートモジュールをトップとしたモジュールのツリーのこと。ルートモジュールが`main.rs`であれば実行バイナリ、それ以外の場合はライブラリを構成する。 |
| (4) | モジュール | クレート内でプログラムを構造化したもの。構造体や関数などのスコープや可視性を制御する。 |
| (5) | 外部クレート | 主に [crates.io](https://crates.io/) で公開されているクレートのこと。 |
| (6) | stdクレート | Rustの標準ライブラリを提供するクレートのこと。 |

![./img/rust_basic.svg](./img/rust_basic.svg)

<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" style="width: 600px; height:600px;">
  <symbol id="folder"><path d="M10,20 l5,-20 50,0 5,20 80,0 5,5 0,80 -5,5 -140,0 -5,-5 0,-80 5,-5 60,0" style="stroke:#000000; fill:#ffffff; stroke-width:1px;" /></symbol>
  <symbol id="node"><rect x="10" y="40" width="100" height="100" style="stroke:#000000; fill:#ffffff; stroke-width: 1px;" /><path d="M10,40 l20,-20 100,0 -20,20" style="stroke:#000000; fill:none; stroke-width: 1px;" /><path d="M131,20 l0,100 -21,20" style="stroke:#000000; fill:none; stroke-width: 1px;" /></symbol>
  <symbol id="file"><path d="M10,10 l80,0 0,90 -10,10 -70,0 0,-100" style="stroke:#000000; fill:none; stroke-width: 1px;" /><path d="M90,100 l-10,0 0,10" style="stroke: #000000; fill:#cccccc; stroke-width: 1px;" /></symbol>
  <use id="package" xlink:href="#folder" x="0" y="5" transform="scale(3.8,5)" />
  <text x="150" y="85" style="font-size: 30px; text-anchor:middle;">(1) Package</text>
  <use id="cargo.toml" xlink:href="#file" x="60" y="160" transform="scale(1.4,0.8)" />
  <text x="155" y="167" style="font-size: 20px; text-anchor:middle;">(2)</text>
  <text x="155" y="187" style="font-size: 20px; text-anchor:middle;">Cargo.toml</text>
  <use id="crate" xlink:href="#node" x="3" y="60" transform="scale(3,2.8)" />
  <text x="185" y="310" style="font-size: 30px; text-anchor:middle;">(3) Crate</text>
  <use id="root_module" xlink:href="#file" x="140" y="310" />
  <text x="190" y="355" style="font-size: 20px; text-anchor:middle;">(4)</text>
  <text x="190" y="375" style="font-size: 20px; text-anchor:middle;">Root</text>
  <text x="190" y="395" style="font-size: 20px; text-anchor:middle;">Module</text>
  <use id="module3" xlink:href="#file" x="240" y="440" />
  <text x="290" y="495" style="font-size: 20px; text-anchor:middle;">(4)</text>
  <text x="290" y="515" style="font-size: 20px; text-anchor:middle;">Module3</text>
  <use id="module2" xlink:href="#file" x="140" y="440" />
  <text x="190" y="495" style="font-size: 20px; text-anchor:middle;">(4)</text>
  <text x="190" y="515" style="font-size: 20px; text-anchor:middle;">Module2</text>
  <use id="module1" xlink:href="#file" x="40" y="440" />
  <text x="90" y="495" style="font-size: 20px; text-anchor:middle;">(4)</text>
  <text x="90" y="515" style="font-size: 20px; text-anchor:middle;">Module1</text>
  <use id="foreign_crate1" xlink:href="#node" x="410" y="140" />
  <text x="470" y="215" style="font-size: 20px; text-anchor:middle;">(5)</text>
  <text x="470" y="235" style="font-size: 20px; text-anchor:middle;">Foreign</text>
  <text x="470" y="255" style="font-size: 20px; text-anchor:middle;">Crate1</text>
  <use id="foreign_crate2" xlink:href="#node" x="410" y="270" />
  <text x="470" y="345" style="font-size: 20px; text-anchor:middle;">(5)</text>
  <text x="470" y="365" style="font-size: 20px; text-anchor:middle;">Foreign</text>
  <text x="470" y="385" style="font-size: 20px; text-anchor:middle;">Crate2</text>
  <use id="std_crate" xlink:href="#node" x="410" y="400" />
  <text x="470" y="475" style="font-size: 20px; text-anchor:middle;">(6)</text>
  <text x="470" y="495" style="font-size: 20px; text-anchor:middle;">std</text>
  <text x="470" y="515" style="font-size: 20px; text-anchor:middle;">Crate</text>
</svg>