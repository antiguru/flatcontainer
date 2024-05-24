window.BENCHMARK_DATA = {
  "lastUpdate": 1716576429014,
  "repoUrl": "https://github.com/antiguru/flatcontainer",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "antiguru@gmail.com",
            "name": "Moritz Hoffmann",
            "username": "antiguru"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2487d331526e6ac7b3e6814cbd7c90d783354c2a",
          "message": "Merge pull request #25 from antiguru/test_bench\n\nFix bench",
          "timestamp": "2024-04-12T10:40:32-04:00",
          "tree_id": "cee1652d2df946f7f50784c674b04521a422d28c",
          "url": "https://github.com/antiguru/flatcontainer/commit/2487d331526e6ac7b3e6814cbd7c90d783354c2a"
        },
        "date": 1712933021250,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 797,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 962,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 961,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1428,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1437,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 366749,
            "range": "± 49168",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 457495,
            "range": "± 63549",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3948507,
            "range": "± 44415",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 374945,
            "range": "± 9338",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4080494,
            "range": "± 26103",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15552459,
            "range": "± 426225",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33357744,
            "range": "± 301284",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3621643,
            "range": "± 27360",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3623166,
            "range": "± 22592",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7625378,
            "range": "± 20478",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3966654,
            "range": "± 29730",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16790623,
            "range": "± 779235",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16712131,
            "range": "± 649131",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1828990,
            "range": "± 19520",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1828487,
            "range": "± 27338",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1999482,
            "range": "± 17420",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4943112,
            "range": "± 53756",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 225778,
            "range": "± 4620",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 186499,
            "range": "± 18916",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 156925,
            "range": "± 3518",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 192546,
            "range": "± 8815",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 213705,
            "range": "± 14349",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 225663,
            "range": "± 4688",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 156582,
            "range": "± 3784",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 159629,
            "range": "± 4835",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 165679,
            "range": "± 5320",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 172284,
            "range": "± 4610",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 226949,
            "range": "± 5236",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 316395,
            "range": "± 5692",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 157390,
            "range": "± 6211",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 319198,
            "range": "± 9915",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 334891,
            "range": "± 7795",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 49196662,
            "range": "± 1151658",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4337537,
            "range": "± 52724",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4277930,
            "range": "± 24915",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4653184,
            "range": "± 45411",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8154263,
            "range": "± 66853",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 51574136,
            "range": "± 1383846",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5269144,
            "range": "± 40922",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5119770,
            "range": "± 44275",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6374985,
            "range": "± 53287",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9643458,
            "range": "± 276504",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 14422246,
            "range": "± 661658",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "antiguru@gmail.com",
            "name": "Moritz Hoffmann",
            "username": "antiguru"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4fe52b54b83a49fe3006e6172fa71f8845b55caf",
          "message": "Replace CopyOnto by Push (#28)\n\n* Adding several implementations that were missing.\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>\r\n\r\n* Replace CopyOnto by Push\r\n\r\nSwap the parameters for CopyOnto. Previously, we'd implement CopyOnto\r\nfor a specific type, specifying the region as a parameter to CopyOnto. This\r\nhas the undesired side-effect that downstream crates cannot implement the\r\ntrait for their own regions due to Rust's orphan rules. Switching the\r\nparameters around (Push<T> for Region) moves a local type into the T0\r\nposition, which is compatible with the orphan rules.\r\n---------\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-24T14:02:49-04:00",
          "tree_id": "4336c7241e61d578ad69f28b0b7b835ee8091f0a",
          "url": "https://github.com/antiguru/flatcontainer/commit/4fe52b54b83a49fe3006e6172fa71f8845b55caf"
        },
        "date": 1716573930241,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 954.75,
            "range": "± 12.57",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1270.83,
            "range": "± 19.90",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 956.88,
            "range": "± 314.93",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1267.53,
            "range": "± 18.22",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1272.71,
            "range": "± 6.60",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 305463.28,
            "range": "± 9006.36",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 414154.55,
            "range": "± 35370.38",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3633483.9,
            "range": "± 28684.13",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 481181.35,
            "range": "± 21663.10",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4368684.5,
            "range": "± 29820.94",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15803212.2,
            "range": "± 433036.49",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32588481.1,
            "range": "± 538717.12",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3611997.8,
            "range": "± 21331.50",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3615172.7,
            "range": "± 26646.91",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7901971.1,
            "range": "± 15726.84",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3966042.8,
            "range": "± 58368.50",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16787459.3,
            "range": "± 474507.01",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16288703.5,
            "range": "± 801588.24",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1817389.8,
            "range": "± 32156.52",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1816047.3,
            "range": "± 33107.10",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1997547.1,
            "range": "± 25093.22",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 5044829,
            "range": "± 114994.88",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 211611.8,
            "range": "± 8826.90",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 180441.17,
            "range": "± 3661.99",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 141412.25,
            "range": "± 7735.12",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 190064.2,
            "range": "± 8449.07",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 203954.64,
            "range": "± 14215.70",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 210808.05,
            "range": "± 10818.55",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 142972.78,
            "range": "± 2968.80",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 142513.3,
            "range": "± 7854.34",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 144771.08,
            "range": "± 6293.98",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 159388.18,
            "range": "± 2498.03",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 211603.27,
            "range": "± 5616.71",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 317198.65,
            "range": "± 14187.09",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 152598.65,
            "range": "± 18017.85",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 315995.9,
            "range": "± 9350.10",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 332916.05,
            "range": "± 12422.38",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 48455784,
            "range": "± 1750438.26",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4285583.3,
            "range": "± 30616.39",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4296519.6,
            "range": "± 32345.09",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4651636,
            "range": "± 46737.35",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8259893.3,
            "range": "± 90744.63",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 50397541.3,
            "range": "± 1465816.28",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5275357.5,
            "range": "± 36102.50",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5068259.7,
            "range": "± 52136.89",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6130636.2,
            "range": "± 41244.77",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9542894.55,
            "range": "± 321367.32",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 12364123.9,
            "range": "± 450435.27",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "41898282+github-actions[bot]@users.noreply.github.com",
            "name": "github-actions[bot]",
            "username": "github-actions[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5ecd0bd1e4c00dbfbc9078b44d73efcf71ca8213",
          "message": "chore: release v0.3.0 (#26)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2024-05-24T14:44:23-04:00",
          "tree_id": "0c6b1077656f3b2cb5c84cf9022918110806905b",
          "url": "https://github.com/antiguru/flatcontainer/commit/5ecd0bd1e4c00dbfbc9078b44d73efcf71ca8213"
        },
        "date": 1716576428724,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 955.04,
            "range": "± 6.61",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 958.35,
            "range": "± 277.42",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1271.13,
            "range": "± 7.60",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1278.88,
            "range": "± 48.57",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1274.38,
            "range": "± 6.40",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 308775.61,
            "range": "± 10395.81",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 495273.47,
            "range": "± 118854.36",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3950432.3,
            "range": "± 25294.91",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 400847.83,
            "range": "± 9712.68",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4362563.7,
            "range": "± 281060.72",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15507096.9,
            "range": "± 392719.05",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33218760.8,
            "range": "± 473788.10",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3618830.7,
            "range": "± 20202.79",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3616349.1,
            "range": "± 21115.45",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7896023.8,
            "range": "± 13434.64",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3965682.9,
            "range": "± 30826.76",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16797151,
            "range": "± 387668.87",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16534626.1,
            "range": "± 523382.50",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1818967.6,
            "range": "± 21248.84",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1821187.7,
            "range": "± 18016.46",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1991792.1,
            "range": "± 45435.60",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4964299.6,
            "range": "± 61093.96",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 216745.64,
            "range": "± 3910.38",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 190012.57,
            "range": "± 23910.15",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 173592.88,
            "range": "± 24265.23",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 195171.55,
            "range": "± 9537.06",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 210529,
            "range": "± 14082.03",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 216993.75,
            "range": "± 2750.97",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 188936.84,
            "range": "± 5216.71",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 187476.51,
            "range": "± 8301.85",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 175501.5,
            "range": "± 21480.41",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 174413.59,
            "range": "± 3654.00",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 217400.76,
            "range": "± 3572.62",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 318009.7,
            "range": "± 4560.06",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 186132.84,
            "range": "± 16775.42",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 318970.6,
            "range": "± 10275.31",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 331018.1,
            "range": "± 5283.02",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 49377997.7,
            "range": "± 2181536.35",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4271487.15,
            "range": "± 30245.75",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4286916.8,
            "range": "± 39705.65",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4637139.7,
            "range": "± 55284.03",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8083108.7,
            "range": "± 86899.84",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 50940164.1,
            "range": "± 1675740.24",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5265586.8,
            "range": "± 44101.27",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5089133.6,
            "range": "± 35069.58",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6674123,
            "range": "± 312202.53",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9543042.5,
            "range": "± 245114.08",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 12524726,
            "range": "± 446151.41",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}