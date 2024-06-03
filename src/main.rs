use rocket::{
    futures::{AsyncBufReadExt, StreamExt},
    response,
    tokio::fs,
};

use rocket::response::content;

#[macro_use]
extern crate rocket;

const NAMES: [&str; 29] = [
    "神里绫人",
    "枫原万叶",
    "迪卢克",
    "嘉明",
    "白术",
    "重云",
    "雷泽",
    "魈",
    "菲米尼",
    "凯亚",
    "五郎",
    "班尼特",
    "莱欧斯利",
    "温迪",
    "阿贝多",
    "艾尔海森",
    "卡维",
    "流浪者",
    "提纳里",
    "托马",
    "荒泷一斗",
    "林尼",
    "鹿野院平藏",
    "赛诺",
    "行秋",
    "那维莱特",
    "米卡",
    "达达利亚",
    "钟离",
];

const TOKENS: [&str; 8] = [
    "CRS988", "97ME6H", "37SGS7", "K7W28L", "ATK889", "2Z87FC", "UB24R6", "XW3Y55",
];

#[get("/<token>/<name1>/<name2>")]
fn compare(token: &str, name1: &str, name2: &str) -> String {
    let mut user = 998244353;
    for i in 0..(TOKENS.len() - 1) {
        if TOKENS[i] == token {
            user = i;
        }
    }
    if user == 998244353 {
        "permission died.".to_string()
    } else {
        let (x, y) = get_next_problem(user as i32);
        if (NAMES[x as usize], NAMES[y as usize]) == (name1, name2)
            || (NAMES[x as usize], NAMES[y as usize]) == (name2, name1)
        {
            submit(user as i32, name1, name2);
            "succeed".to_string()
        } else {
            "not the point.".to_string()
        }
    }
}

const TEMPLATE: &str = "<script>
function sendmessssage(g) {
    var httpRequest = new XMLHttpRequest();
    httpRequest.open('GET', g, true);
    httpRequest.send();

    httpRequest.onreadystatechange = function () {
        location.reload();
    };
}
</script>
<button onclick='sendmessssage(window.location.href + \"/##A/##B\")'>##A</button>
<button onclick='sendmessssage(window.location.href + \"/##B/##A\")'>##B</button>
<button onclick='sendmessssage(window.location.href + \"/back\")'>back</button>";

static mut ORDER: [[bool; 29]; 29] = [[false; 29]; 29];
static mut BREAK: bool = false;
static mut LEFT: i32 = 0;
static mut RIGHT: i32 = 0;

pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        merge_sort_range(arr, 0, arr.len() - 1);
    }
}

fn merge_sort_range(arr: &mut [i32], lo: usize, hi: usize) {
    // 只有当元素个数大于一时才进行排序
    if lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        merge_sort_range(arr, lo, mid);
        merge_sort_range(arr, mid + 1, hi);
        merge_two_arrays(arr, lo, mid, hi);
    }
}

// 合并两个有序数组: arr[lo..=mid], arr[mid + 1..=hi]
fn merge_two_arrays(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    unsafe {
        if BREAK {
            return;
        }
    }
    // 这里需要 clone 数组元素
    let mut arr1 = arr[lo..=mid].to_vec();
    let mut arr2 = arr[mid + 1..=hi].to_vec();

    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        unsafe {
            if ORDER[arr1[i] as usize][arr2[j] as usize] {
                arr[i + j + lo] = std::mem::take(&mut arr1[i]);
                i += 1;
            } else if ORDER[arr2[j] as usize][arr1[i] as usize] {
                arr[i + j + lo] = std::mem::take(&mut arr2[j]);
                j += 1;
            } else {
                BREAK = true;
                LEFT = arr1[i];
                RIGHT = arr2[j];
                return;
            }
        }
    }

    while i < arr1.len() {
        arr[i + j + lo] = std::mem::take(&mut arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        arr[i + j + lo] = std::mem::take(&mut arr2[j]);
        j += 1;
    }
}

#[get("/<token>")]
fn page(token: &str) -> content::RawHtml<String> {
    let mut user = 998244353;
    for i in 0..(TOKENS.len() - 1) {
        if TOKENS[i] == token {
            user = i;
        }
    }
    if user == 998244353 {
        content::RawHtml("permission died.".to_string())
    } else {
        let (x, y) = get_next_problem(user as i32);
        if x == -1 {
            content::RawHtml(
                std::fs::read_to_string(format!("{user}list.txt")).unwrap_or("".to_string()),
            )
        } else {
            let res = String::from(TEMPLATE);
            content::RawHtml(
                res.replace("##A", NAMES[x as usize])
                    .replace("##B", NAMES[y as usize]),
            )
        }
    }
}

fn get_next_problem(user: i32) -> (i32, i32) {
    let list = std::fs::read(format!("{user}list.txt"));
    match list {
        Ok(_) => (-1, -1),
        Err(_) => unsafe {
            let answers = std::fs::read(format!("{user}answers.txt")).unwrap_or("".into());

            String::from_utf8(answers).unwrap().lines().for_each(|f| {
                let k: Vec<&str> = f.split(" ").collect();
                ORDER[k[0].parse::<usize>().unwrap() as usize]
                    [k[1].parse::<usize>().unwrap() as usize] = true;
            });

            for k in 0..29 {
                for i in 0..29 {
                    for j in 0..29 {
                        ORDER[i][j] = ORDER[i][j] || (ORDER[i][k] && ORDER[j][k]);
                    }
                }
            }

            let mut arr = [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28,
            ];

            LEFT = -1;
            RIGHT = -1;

            merge_sort(&mut arr);

            (LEFT, RIGHT)
        },
    }
}

fn submit(user: i32, name1: &str, name2: &str) {
    let mut answers =
        String::from_utf8(std::fs::read(format!("{user}answers.txt")).unwrap_or("".into()))
            .unwrap();
    let mut a = 0;
    let mut b = 0;
    for i in 0..NAMES.len() {
        if NAMES[i] == name1 {
            a = i;
        } else if NAMES[i] == name2 {
            b = i;
        }
    }
    answers.push_str(&format!("{a} {b}"));
    let _ = std::fs::write(format!("{user}answers.txt"), answers);
}

#[get("/")]
fn index() -> String {
    "required token".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![compare, index, page])
}
