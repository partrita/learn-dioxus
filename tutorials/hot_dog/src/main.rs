use dioxus::prelude::*;

// 전역 CSS, 파비콘 에셋 선언 (`/`는 프로젝트 루트 기준)
static CSS: Asset = asset!("/assets/main.css");
static ICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

// 최상위 컴포넌트: 스타일 + 제목 + 강아지 뷰 조립
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

// 상단 제목바
#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

// dog.ceo API 응답 모양 (message = 이미지 URL)
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

// 강아지 사진 1장 + skip/save 버튼
#[component]
fn DogView() -> Element {
    // use_resource: 비동기 작업 결과를 담는 상태. 처음에 1번 실행됨
    let mut img_src = use_resource(|| async move {
        // 랜덤 강아지 이미지 1장 요청 -> JSON -> URL 문자열만 꺼냄
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            // 아직 로딩 중이면(None) 빈 문자열 -> 이미지 없음 표시
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            // restart(): resource를 다시 실행해서 새 사진 가져옴
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}
