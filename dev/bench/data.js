window.BENCHMARK_DATA = {
  "lastUpdate": 1718655001306,
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
          "id": "35a3b0734b91b5faa0f74d1a2f8e7be8ac5e5324",
          "message": "Update recommended version to 0.3 (#29)\n\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-24T16:09:27-04:00",
          "tree_id": "d7676dd1ac22107187e05ab96a4ea08e19cae995",
          "url": "https://github.com/antiguru/flatcontainer/commit/35a3b0734b91b5faa0f74d1a2f8e7be8ac5e5324"
        },
        "date": 1716581497341,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 955.03,
            "range": "± 4.81",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 969.56,
            "range": "± 251.13",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1272,
            "range": "± 33.10",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1330.78,
            "range": "± 234.95",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1328.35,
            "range": "± 834.29",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 307175.65,
            "range": "± 12349.91",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 407057.5,
            "range": "± 21524.05",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3954618.7,
            "range": "± 26356.57",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 530718.6,
            "range": "± 8882.56",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4386759.1,
            "range": "± 22916.65",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15519850,
            "range": "± 529775.41",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33106644.4,
            "range": "± 627222.61",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3618593.7,
            "range": "± 30126.17",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3611898.6,
            "range": "± 20577.78",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7577493.7,
            "range": "± 46786.68",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3964298.7,
            "range": "± 20628.56",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16292013,
            "range": "± 280697.90",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16451295.9,
            "range": "± 349809.10",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1814744.8,
            "range": "± 14883.32",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1815922.9,
            "range": "± 20877.70",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1993645.2,
            "range": "± 18155.30",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4908640.8,
            "range": "± 58085.25",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 209759.75,
            "range": "± 3177.04",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 183930.67,
            "range": "± 14470.20",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 143155.47,
            "range": "± 6642.27",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 188491.97,
            "range": "± 7512.67",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 204606.97,
            "range": "± 11653.67",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 210116.27,
            "range": "± 5662.22",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 143180.02,
            "range": "± 3892.76",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 143148.92,
            "range": "± 5646.31",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 145702.59,
            "range": "± 2187.02",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 158765.4,
            "range": "± 3664.05",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 209771.97,
            "range": "± 5312.33",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 318322.25,
            "range": "± 4643.08",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 142491.48,
            "range": "± 6556.47",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 317101,
            "range": "± 4619.98",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 330616.35,
            "range": "± 5146.15",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 48025783.9,
            "range": "± 1585307.23",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4288585.7,
            "range": "± 48311.04",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4290573.2,
            "range": "± 37918.75",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4644874.6,
            "range": "± 24273.93",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8212514.6,
            "range": "± 66267.76",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 48891939,
            "range": "± 2704319.24",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5251814.6,
            "range": "± 29921.60",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5086186.9,
            "range": "± 44749.74",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6663639.9,
            "range": "± 46367.85",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9333973.5,
            "range": "± 202441.60",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11890537.8,
            "range": "± 566425.27",
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
          "id": "1b1bc724ccad8651292e09bfb7f3d03e1d9d77c6",
          "message": "chore: release v0.3.1 (#30)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2024-05-24T16:17:49-04:00",
          "tree_id": "c7ecef9fb93e2c92cbbc870a20cd4afaa4d4ab5e",
          "url": "https://github.com/antiguru/flatcontainer/commit/1b1bc724ccad8651292e09bfb7f3d03e1d9d77c6"
        },
        "date": 1716582029657,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 954.9,
            "range": "± 12.30",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1273.36,
            "range": "± 36.03",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1271.59,
            "range": "± 11.32",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1278.06,
            "range": "± 21.86",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1279.49,
            "range": "± 23.01",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 303278.38,
            "range": "± 17721.73",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 402426.51,
            "range": "± 46583.41",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3633637.9,
            "range": "± 23918.80",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 484960.03,
            "range": "± 4175.27",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4047323,
            "range": "± 24668.89",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15820787.4,
            "range": "± 664585.91",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32488931.3,
            "range": "± 464056.53",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3624833.8,
            "range": "± 63735.44",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3616613.4,
            "range": "± 24371.18",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7577051.1,
            "range": "± 35477.76",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3962684.9,
            "range": "± 17945.69",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 14665604.8,
            "range": "± 518933.66",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16138850.5,
            "range": "± 504593.16",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1821907.6,
            "range": "± 18857.54",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1816613.8,
            "range": "± 27294.75",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1991443.7,
            "range": "± 14724.13",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4975042,
            "range": "± 88680.48",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 205254.25,
            "range": "± 2502.63",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 184530.16,
            "range": "± 7159.42",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 142433.62,
            "range": "± 2248.24",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 189980.3,
            "range": "± 12767.31",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 204610.57,
            "range": "± 6415.78",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 205982,
            "range": "± 63856.68",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 142577.5,
            "range": "± 6727.83",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 142415.65,
            "range": "± 3397.59",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 143973.74,
            "range": "± 4599.61",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 157581.58,
            "range": "± 4867.17",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 204936.32,
            "range": "± 4874.01",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 315584.95,
            "range": "± 6601.79",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 142973.18,
            "range": "± 3651.65",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 318272.4,
            "range": "± 4669.02",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 330516.15,
            "range": "± 7146.53",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 48076923.2,
            "range": "± 1311182.78",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4281464.1,
            "range": "± 25802.92",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4274930.5,
            "range": "± 40747.05",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4633982.6,
            "range": "± 39032.08",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8138094.25,
            "range": "± 66261.26",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 51030176.8,
            "range": "± 1633085.51",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5277906.8,
            "range": "± 51699.07",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5070737.8,
            "range": "± 59130.02",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 5928030.3,
            "range": "± 158929.97",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9628734.45,
            "range": "± 419557.63",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 12362907.3,
            "range": "± 530818.63",
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
          "id": "76769820c09278b8d49e4062be1eee588fa71eec",
          "message": "Introduce reborrow to enable lifetime variance (#32)\n\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-27T14:16:15-04:00",
          "tree_id": "037cfd527c1070637bcbbc35846c27de5973c075",
          "url": "https://github.com/antiguru/flatcontainer/commit/76769820c09278b8d49e4062be1eee588fa71eec"
        },
        "date": 1716833942418,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 955.54,
            "range": "± 26.46",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1271.64,
            "range": "± 16.91",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1270.78,
            "range": "± 8.06",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1271.3,
            "range": "± 9.06",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1285.81,
            "range": "± 36.68",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 309737.34,
            "range": "± 63802.30",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 410496.47,
            "range": "± 77290.94",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3955938.2,
            "range": "± 23395.26",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 470090.62,
            "range": "± 43002.89",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4057491.3,
            "range": "± 34992.00",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15372005.4,
            "range": "± 807546.68",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32510355.3,
            "range": "± 550603.84",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3938569.1,
            "range": "± 22944.03",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3940937.7,
            "range": "± 30750.44",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7255307.4,
            "range": "± 93210.75",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 4294860.3,
            "range": "± 173419.99",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16253048,
            "range": "± 699025.25",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16182276.9,
            "range": "± 803378.39",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1978429.7,
            "range": "± 15832.30",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1985176.4,
            "range": "± 60326.75",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 2153401.1,
            "range": "± 7783.73",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4754719.6,
            "range": "± 57593.26",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 207025.56,
            "range": "± 2371.00",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 188457.17,
            "range": "± 17047.39",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 145271.11,
            "range": "± 4061.90",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 189986.62,
            "range": "± 13504.10",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 205136.64,
            "range": "± 10787.00",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 206888.38,
            "range": "± 3574.83",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 144147.59,
            "range": "± 1618.31",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 143997.33,
            "range": "± 1842.50",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 144182.03,
            "range": "± 1999.01",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 159613.36,
            "range": "± 9486.99",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 206956.04,
            "range": "± 2785.69",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 315582.95,
            "range": "± 4205.73",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 144417.2,
            "range": "± 3625.55",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 318620.2,
            "range": "± 5948.97",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 331372.15,
            "range": "± 5867.63",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 46869312.2,
            "range": "± 1712230.77",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4276640.95,
            "range": "± 44083.36",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4268005.95,
            "range": "± 24848.00",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4633120,
            "range": "± 34639.51",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8222544.9,
            "range": "± 60727.32",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 49493282,
            "range": "± 1883835.27",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5106243.9,
            "range": "± 49376.88",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5080403.1,
            "range": "± 69806.02",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 5865853.1,
            "range": "± 325160.01",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9223883.45,
            "range": "± 366771.32",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11773918.6,
            "range": "± 463045.12",
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
          "id": "2ed12044e3470a8e8c58eb5259196cda3228b33b",
          "message": "Thinking about relating owned types and read items (#31)\n\nAdd IntoOwned trait, add Owned type to region, add a constraint that read items must be IntoOwned.\r\n\r\n---------\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-28T16:52:03-04:00",
          "tree_id": "22d15ff87e4f691fb63643ba46b39b67303a19c8",
          "url": "https://github.com/antiguru/flatcontainer/commit/2ed12044e3470a8e8c58eb5259196cda3228b33b"
        },
        "date": 1716929695261,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 955.65,
            "range": "± 10.95",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 995.06,
            "range": "± 310.90",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1271.92,
            "range": "± 15.40",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1284.13,
            "range": "± 23.76",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1285.32,
            "range": "± 20.04",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 369814.17,
            "range": "± 43323.50",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 466131.39,
            "range": "± 55053.09",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3643332.75,
            "range": "± 16661.33",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 453273.83,
            "range": "± 8176.12",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4396561.6,
            "range": "± 55425.36",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 16705509.9,
            "range": "± 316335.39",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33085492.3,
            "range": "± 523234.63",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3635585.8,
            "range": "± 19482.55",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3628937.4,
            "range": "± 33062.31",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7909307.1,
            "range": "± 22587.63",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3977123,
            "range": "± 21183.86",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 17724395.3,
            "range": "± 209281.49",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16821454.4,
            "range": "± 388938.13",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1832269.3,
            "range": "± 23834.85",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1832256.95,
            "range": "± 72579.38",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 2007833.1,
            "range": "± 43151.68",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 5158643.5,
            "range": "± 54722.96",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 224735.84,
            "range": "± 5078.03",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 182915.64,
            "range": "± 3948.43",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 159289.6,
            "range": "± 3013.29",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 198040.03,
            "range": "± 7954.60",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 215443.7,
            "range": "± 15129.45",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 222412.8,
            "range": "± 6237.03",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 161678.75,
            "range": "± 4346.75",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 159052.83,
            "range": "± 3993.96",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 162995.71,
            "range": "± 3734.48",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 176306.33,
            "range": "± 3507.35",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 222470.58,
            "range": "± 4855.06",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 316948.95,
            "range": "± 7109.89",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 158954.37,
            "range": "± 3126.18",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 318479.05,
            "range": "± 7704.17",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 335695.78,
            "range": "± 4408.58",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 50569076.2,
            "range": "± 965911.84",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4283227.45,
            "range": "± 98829.80",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4283993.4,
            "range": "± 28866.95",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4642167.7,
            "range": "± 29220.93",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8497886.2,
            "range": "± 122268.38",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 52828620.5,
            "range": "± 1371016.34",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5316600.9,
            "range": "± 43705.48",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5186052.7,
            "range": "± 92063.21",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6204933.5,
            "range": "± 162935.15",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9954646.05,
            "range": "± 320877.94",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 13391576,
            "range": "± 449137.39",
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
          "id": "7f45ad83483348e3bfe1df436e5d3ed4205fce7e",
          "message": "chore: release v0.4.0 (#33)\n\n* chore: release\r\n\r\n* Update version\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>\r\n\r\n---------\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>\r\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>\r\nCo-authored-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-28T17:02:01-04:00",
          "tree_id": "024bc94b33d0ad65797a4633fc22c73521342a3c",
          "url": "https://github.com/antiguru/flatcontainer/commit/7f45ad83483348e3bfe1df436e5d3ed4205fce7e"
        },
        "date": 1716930252090,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 1011.48,
            "range": "± 7.99",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 958.88,
            "range": "± 260.36",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 956.65,
            "range": "± 261.45",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1275.06,
            "range": "± 29.37",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1285.72,
            "range": "± 1209.50",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 304070.9,
            "range": "± 7565.23",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 404806.9,
            "range": "± 29921.50",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3947143.6,
            "range": "± 25209.73",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 476248.1,
            "range": "± 17585.21",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4360256.9,
            "range": "± 38263.85",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15798559.7,
            "range": "± 210718.62",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32362355.2,
            "range": "± 1341533.19",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3670475.2,
            "range": "± 16565.17",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3665844.5,
            "range": "± 32386.04",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7604305.9,
            "range": "± 142014.85",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 4359875.1,
            "range": "± 22698.07",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16388784.4,
            "range": "± 963834.42",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 15934259.7,
            "range": "± 492821.38",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1852938.4,
            "range": "± 19964.97",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1855043.3,
            "range": "± 11402.73",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 2228970,
            "range": "± 28893.01",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4968925.4,
            "range": "± 34171.45",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 248777.1,
            "range": "± 5141.03",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 182469.95,
            "range": "± 13208.83",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 175994.35,
            "range": "± 38874.90",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 188960.4,
            "range": "± 14212.82",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 202887.19,
            "range": "± 9558.07",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 250122.43,
            "range": "± 5963.98",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 178420.88,
            "range": "± 3803.57",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 177433.43,
            "range": "± 5917.25",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 179632.38,
            "range": "± 2993.96",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 193415.3,
            "range": "± 5539.67",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 248884.55,
            "range": "± 15348.58",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 319335.25,
            "range": "± 6747.51",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 177187.23,
            "range": "± 4224.04",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 319312.45,
            "range": "± 7633.77",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 337844.7,
            "range": "± 13396.19",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 46344382,
            "range": "± 686874.42",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4257490.25,
            "range": "± 35943.86",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4257684.1,
            "range": "± 31682.19",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4636850.2,
            "range": "± 905699.67",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8393006.9,
            "range": "± 58132.29",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 49039072.8,
            "range": "± 966078.61",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5266564,
            "range": "± 30795.57",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5077734.9,
            "range": "± 55668.80",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6060748.8,
            "range": "± 97401.08",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9507440.6,
            "range": "± 580437.43",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 12349125.9,
            "range": "± 412021.87",
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
          "id": "c89c35573ad21237e8ebccc9677f6c9bf53b2f68",
          "message": "Move complex tests to separate folder (#34)\n\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-05-31T13:46:13-04:00",
          "tree_id": "8d016a29fcbe82ae87428f4dfd76db41baa0b676",
          "url": "https://github.com/antiguru/flatcontainer/commit/c89c35573ad21237e8ebccc9677f6c9bf53b2f68"
        },
        "date": 1717177722880,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 954.68,
            "range": "± 9.87",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1120.92,
            "range": "± 311.60",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 1082.47,
            "range": "± 308.13",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1283.33,
            "range": "± 33.47",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1274.4,
            "range": "± 33.75",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 309964.45,
            "range": "± 11903.54",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 421632.1,
            "range": "± 34110.91",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3947126.7,
            "range": "± 20233.85",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 405073.07,
            "range": "± 11280.40",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4367406.7,
            "range": "± 343569.91",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 14988054,
            "range": "± 435408.78",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32726829.3,
            "range": "± 561384.98",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3626706.8,
            "range": "± 29131.51",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3625201.9,
            "range": "± 24583.69",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7576059.6,
            "range": "± 86225.62",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3979558,
            "range": "± 18649.24",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16927540,
            "range": "± 455881.28",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16296920.2,
            "range": "± 567929.14",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1818517.6,
            "range": "± 14591.23",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1819847.6,
            "range": "± 19013.68",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1997409.2,
            "range": "± 34440.34",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 5112587.8,
            "range": "± 113896.11",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 213536.43,
            "range": "± 4809.98",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 181125.57,
            "range": "± 7066.07",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 144226.8,
            "range": "± 4006.63",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 183638.1,
            "range": "± 8220.53",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 204205.77,
            "range": "± 19909.49",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 213841,
            "range": "± 3079.80",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 143693.2,
            "range": "± 5112.89",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 143532.88,
            "range": "± 8139.67",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 145432.34,
            "range": "± 4929.98",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 158063.25,
            "range": "± 6817.52",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 213803.05,
            "range": "± 10179.82",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 318443.25,
            "range": "± 6294.31",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 144477.69,
            "range": "± 3124.13",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 317412.2,
            "range": "± 7128.76",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 334912.25,
            "range": "± 7318.07",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 48674605.6,
            "range": "± 1424716.62",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4281819,
            "range": "± 59030.25",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4275702,
            "range": "± 35462.67",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4628427.6,
            "range": "± 33130.93",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8175117,
            "range": "± 122854.25",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 51198299.2,
            "range": "± 1478941.34",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5078437,
            "range": "± 42582.64",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5067346.4,
            "range": "± 42591.47",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6391945,
            "range": "± 47230.97",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9133216.4,
            "range": "± 206777.38",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11872737.6,
            "range": "± 488649.01",
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
          "id": "04d5c636285c0e9d51bd888914f922b5cbe9240d",
          "message": "Fix warning on Rust 1.79 (#38)\n\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-06-14T15:06:50-04:00",
          "tree_id": "2b58282192561ffe515174d3e3d289e716bb4b27",
          "url": "https://github.com/antiguru/flatcontainer/commit/04d5c636285c0e9d51bd888914f922b5cbe9240d"
        },
        "date": 1718392177176,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 1019.57,
            "range": "± 83.71",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 979.14,
            "range": "± 263.91",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 964.02,
            "range": "± 308.33",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1318.86,
            "range": "± 37.86",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1321.87,
            "range": "± 38.79",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 326959.7,
            "range": "± 33505.64",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 456953.87,
            "range": "± 60015.74",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3636656.6,
            "range": "± 142372.61",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 354549.92,
            "range": "± 10027.03",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4049385.7,
            "range": "± 27814.36",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 16040646.5,
            "range": "± 2049841.90",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33504397.5,
            "range": "± 322818.61",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3632738.3,
            "range": "± 41504.72",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3620022.3,
            "range": "± 29151.99",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7900583.1,
            "range": "± 24675.95",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3969222.1,
            "range": "± 19471.62",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16928281.8,
            "range": "± 526321.10",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16782767.2,
            "range": "± 505470.93",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1820922.1,
            "range": "± 19049.54",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1819979.4,
            "range": "± 20645.72",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1998100.05,
            "range": "± 17460.08",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4906466.4,
            "range": "± 79027.70",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 212024.35,
            "range": "± 16376.25",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 184286.13,
            "range": "± 13744.22",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 145244.91,
            "range": "± 2115.46",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 189814.98,
            "range": "± 7664.38",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 207692.38,
            "range": "± 12389.38",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 213502.95,
            "range": "± 6388.54",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 142253.17,
            "range": "± 3612.44",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 145038.15,
            "range": "± 4640.18",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 144842.59,
            "range": "± 31932.58",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 158964.05,
            "range": "± 3797.29",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 213823.47,
            "range": "± 8263.95",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 318494.15,
            "range": "± 4314.24",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 144991.27,
            "range": "± 1734.71",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 321726.7,
            "range": "± 6028.51",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 333901.8,
            "range": "± 4841.57",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 48563550.8,
            "range": "± 766828.00",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4247889.45,
            "range": "± 29100.08",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4252299.4,
            "range": "± 248564.62",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4622479.8,
            "range": "± 279922.88",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8082319.4,
            "range": "± 509442.72",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 52465920.6,
            "range": "± 4341408.30",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5257082.2,
            "range": "± 37644.86",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5072012.3,
            "range": "± 41195.92",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 5881179,
            "range": "± 29387.73",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9789098.25,
            "range": "± 297338.88",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 12592920.6,
            "range": "± 723824.66",
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
          "id": "889b7a432c17e7826cace0557540bdb88e9dce13",
          "message": "Huffman container (#20)\n\nHuffman container\r\n\r\n---------\r\n\r\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>\r\nCo-authored-by: Frank McSherry <fmcsherry@me.com>",
          "timestamp": "2024-06-14T15:13:32-04:00",
          "tree_id": "48707f95a6899b334d4723a16c23fea5227f7f00",
          "url": "https://github.com/antiguru/flatcontainer/commit/889b7a432c17e7826cace0557540bdb88e9dce13"
        },
        "date": 1718392542188,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 954.55,
            "range": "± 8.78",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 956.16,
            "range": "± 192.48",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 955.93,
            "range": "± 244.87",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1300.71,
            "range": "± 54.95",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1280.29,
            "range": "± 19.72",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 301093.45,
            "range": "± 6341.51",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 400145.8,
            "range": "± 21675.68",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3632767.8,
            "range": "± 88779.19",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 475172.95,
            "range": "± 4096.44",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4043741.9,
            "range": "± 15677.08",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15040786.8,
            "range": "± 983568.31",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 33408132,
            "range": "± 906663.98",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3614801.3,
            "range": "± 177687.62",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3614048,
            "range": "± 19821.45",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7901522.9,
            "range": "± 16724.93",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 4032702.3,
            "range": "± 315366.28",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 17441600.3,
            "range": "± 1362153.72",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16202998.6,
            "range": "± 282845.39",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1812907.7,
            "range": "± 39003.60",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1813628.05,
            "range": "± 22965.16",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1984908.8,
            "range": "± 7178.93",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4971014.2,
            "range": "± 100420.75",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 209735.32,
            "range": "± 1712.16",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 184436.23,
            "range": "± 13483.14",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 139741.3,
            "range": "± 1483.31",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 188934.65,
            "range": "± 8707.33",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 206681.77,
            "range": "± 6890.87",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 210169.47,
            "range": "± 2797.85",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 141120.4,
            "range": "± 4777.17",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 140051.04,
            "range": "± 1115.09",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 142031.32,
            "range": "± 1734.21",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 159509.32,
            "range": "± 6056.02",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 210235.43,
            "range": "± 4199.34",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 320814.8,
            "range": "± 5142.58",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 140054.83,
            "range": "± 1707.79",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 317522.6,
            "range": "± 3187.03",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 337998.95,
            "range": "± 2448.82",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 46696322.5,
            "range": "± 23234562.06",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4253831.1,
            "range": "± 18677.45",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4256371,
            "range": "± 26832.03",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4603272.6,
            "range": "± 26160.77",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8143897.7,
            "range": "± 35796.62",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 46654148.3,
            "range": "± 337349.42",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5243153.8,
            "range": "± 30997.90",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5051534.2,
            "range": "± 31496.69",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 5882414.1,
            "range": "± 32069.50",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9158182.7,
            "range": "± 138405.96",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11815044,
            "range": "± 126175.83",
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
          "id": "ee7826b8c93823d423600a837dd0978cdf4f4a8c",
          "message": "Add missing Ord and ReserveItems impls (#39)\n\nSigned-off-by: Moritz Hoffmann <antiguru@gmail.com>",
          "timestamp": "2024-06-17T15:59:54-04:00",
          "tree_id": "0cdb04cc284685facf25e84f40f2f9ffcfe23944",
          "url": "https://github.com/antiguru/flatcontainer/commit/ee7826b8c93823d423600a837dd0978cdf4f4a8c"
        },
        "date": 1718654547549,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 954.99,
            "range": "± 16.29",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1271.07,
            "range": "± 20.20",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 956.99,
            "range": "± 253.45",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1283.09,
            "range": "± 197.40",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1282.36,
            "range": "± 22.45",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 304683.15,
            "range": "± 5633.44",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 416608.9,
            "range": "± 48568.23",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3958694.2,
            "range": "± 339996.71",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 400959.25,
            "range": "± 9124.47",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4063649.1,
            "range": "± 18372.43",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 14840468.8,
            "range": "± 252494.94",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32266329.8,
            "range": "± 628120.13",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3938210.8,
            "range": "± 22169.70",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3938339.1,
            "range": "± 17382.71",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7258203.7,
            "range": "± 26274.84",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 3962054.1,
            "range": "± 35971.39",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 14710311.7,
            "range": "± 407674.75",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 15875384.2,
            "range": "± 477120.08",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1983180,
            "range": "± 15500.99",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1979361.9,
            "range": "± 15230.03",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 1994997.2,
            "range": "± 14061.89",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4716856.1,
            "range": "± 70502.41",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 212404,
            "range": "± 5956.84",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 182443.23,
            "range": "± 3821.56",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 143571.5,
            "range": "± 6319.24",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 181922.08,
            "range": "± 4599.05",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 211863.83,
            "range": "± 21221.34",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 211112.4,
            "range": "± 5822.35",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 170575.58,
            "range": "± 2912.55",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 169822.08,
            "range": "± 6389.30",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 172102.25,
            "range": "± 4798.77",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 159775.23,
            "range": "± 2276.32",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 211794.33,
            "range": "± 4153.17",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 319800.45,
            "range": "± 15960.59",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 169707.77,
            "range": "± 28741.11",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 317406.67,
            "range": "± 13996.88",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 336782.1,
            "range": "± 9700.98",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 46359600.8,
            "range": "± 1942207.53",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4265003.5,
            "range": "± 30072.48",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4269245.6,
            "range": "± 29738.59",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4679209.25,
            "range": "± 120944.85",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8270380.7,
            "range": "± 61194.22",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 49026451.1,
            "range": "± 2346172.29",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5335244.3,
            "range": "± 40068.14",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5096970.5,
            "range": "± 19068.53",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6616163.2,
            "range": "± 31690.46",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9380863.7,
            "range": "± 605079.12",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11970000.3,
            "range": "± 391255.71",
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
          "id": "ef8912c9f12b7c1ebaed52491ee1364c529f6734",
          "message": "chore: release v0.4.1 (#35)\n\nCo-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>",
          "timestamp": "2024-06-17T16:07:40-04:00",
          "tree_id": "34b86addcc5c7a9ea154cc48afbe6a90cb4f592e",
          "url": "https://github.com/antiguru/flatcontainer/commit/ef8912c9f12b7c1ebaed52491ee1364c529f6734"
        },
        "date": 1718655000877,
        "tool": "cargo",
        "benches": [
          {
            "name": "empty_clone",
            "value": 956.49,
            "range": "± 28.94",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy",
            "value": 1271.2,
            "range": "± 11.07",
            "unit": "ns/iter"
          },
          {
            "name": "empty_copy_region",
            "value": 961.11,
            "range": "± 263.12",
            "unit": "ns/iter"
          },
          {
            "name": "empty_prealloc",
            "value": 1312.93,
            "range": "± 34.28",
            "unit": "ns/iter"
          },
          {
            "name": "empty_realloc",
            "value": 1285.95,
            "range": "± 17.94",
            "unit": "ns/iter"
          },
          {
            "name": "str100_copy_region",
            "value": 349488.05,
            "range": "± 28328.34",
            "unit": "ns/iter"
          },
          {
            "name": "str10_clone",
            "value": 430904.5,
            "range": "± 30171.94",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy",
            "value": 3961610.5,
            "range": "± 44869.46",
            "unit": "ns/iter"
          },
          {
            "name": "str10_copy_region",
            "value": 543024.9,
            "range": "± 11718.78",
            "unit": "ns/iter"
          },
          {
            "name": "str10_prealloc",
            "value": 4058478,
            "range": "± 25923.61",
            "unit": "ns/iter"
          },
          {
            "name": "str10_realloc",
            "value": 15472895.8,
            "range": "± 582630.56",
            "unit": "ns/iter"
          },
          {
            "name": "string10_clone",
            "value": 32543417.6,
            "range": "± 961309.01",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy",
            "value": 3624310.2,
            "range": "± 24616.00",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region",
            "value": 3624193.9,
            "range": "± 34366.43",
            "unit": "ns/iter"
          },
          {
            "name": "string10_copy_region_collapse",
            "value": 7255480.1,
            "range": "± 20568.04",
            "unit": "ns/iter"
          },
          {
            "name": "string10_prealloc",
            "value": 4285046.6,
            "range": "± 33613.07",
            "unit": "ns/iter"
          },
          {
            "name": "string10_realloc",
            "value": 16764800.1,
            "range": "± 481155.78",
            "unit": "ns/iter"
          },
          {
            "name": "string20_clone",
            "value": 16163395.8,
            "range": "± 525118.16",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy",
            "value": 1822762.8,
            "range": "± 27057.17",
            "unit": "ns/iter"
          },
          {
            "name": "string20_copy_region",
            "value": 1820789.6,
            "range": "± 10609.11",
            "unit": "ns/iter"
          },
          {
            "name": "string20_prealloc",
            "value": 2314866.5,
            "range": "± 183896.39",
            "unit": "ns/iter"
          },
          {
            "name": "string20_realloc",
            "value": 4770820.5,
            "range": "± 40513.88",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_clone",
            "value": 216793.18,
            "range": "± 5180.00",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy",
            "value": 180671.63,
            "range": "± 9911.99",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_copy_region",
            "value": 153410.35,
            "range": "± 8496.77",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_prealloc",
            "value": 193163.25,
            "range": "± 7425.41",
            "unit": "ns/iter"
          },
          {
            "name": "u32x2_realloc",
            "value": 196748.4,
            "range": "± 7926.05",
            "unit": "ns/iter"
          },
          {
            "name": "u64_clone",
            "value": 217755.4,
            "range": "± 9829.44",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy",
            "value": 155798.33,
            "range": "± 6048.23",
            "unit": "ns/iter"
          },
          {
            "name": "u64_copy_region",
            "value": 155496.22,
            "range": "± 8028.05",
            "unit": "ns/iter"
          },
          {
            "name": "u64_prealloc",
            "value": 156515.62,
            "range": "± 7558.38",
            "unit": "ns/iter"
          },
          {
            "name": "u64_realloc",
            "value": 170801.55,
            "range": "± 9769.35",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_clone",
            "value": 216998.31,
            "range": "± 5716.76",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy",
            "value": 314681.35,
            "range": "± 7155.68",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_copy_region",
            "value": 154525.16,
            "range": "± 4060.58",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_prealloc",
            "value": 318601.65,
            "range": "± 6945.55",
            "unit": "ns/iter"
          },
          {
            "name": "u8_u64_realloc",
            "value": 334553.4,
            "range": "± 9361.74",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_clone",
            "value": 47837471.1,
            "range": "± 1451673.64",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy",
            "value": 4277882.85,
            "range": "± 37619.11",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_copy_region",
            "value": 4277161.1,
            "range": "± 48749.73",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_prealloc",
            "value": 4647834.9,
            "range": "± 63350.42",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_s_realloc",
            "value": 8275306.1,
            "range": "± 56099.12",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_clone",
            "value": 49745630.3,
            "range": "± 1521810.87",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy",
            "value": 5123930.4,
            "range": "± 36139.22",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region",
            "value": 5109805,
            "range": "± 40007.22",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_copy_region_column",
            "value": 6338678.4,
            "range": "± 43900.62",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_prealloc",
            "value": 9154679.4,
            "range": "± 245564.78",
            "unit": "ns/iter"
          },
          {
            "name": "vec_u_vn_s_realloc",
            "value": 11729335.8,
            "range": "± 412067.21",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}