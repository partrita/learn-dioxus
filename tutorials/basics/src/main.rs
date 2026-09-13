use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

// 최상위: 테마 제공 + 8개 데모 조립
#[component]
fn App() -> Element {
    // Context API: 자식 전체에 Signal 하나 공유
    use_context_provider(|| use_signal(|| "light".to_string()));

    rsx! {
        Counter {}
        Greeting { name: "디옥서스".to_string() }
        ThemeReader {}
        ListDemo {}
        TextInput {}
        Toggle {}
        EffectDemo {}
        AsyncDemo {}
    }
}

// 1. use_signal + use_memo: 지역 상태와 파생값
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    // count를 읽으므로 count가 바뀔 때만 재계산
    let doubled = use_memo(move || count() * 2);

    rsx! {
        h1 { "Count: {count} / 2배: {doubled}" }
        button { onclick: move |_| *count.write() += 1, "+1" }
    }
}

// 2. Props: 부모 → 자식 값 전달 (owned String, PartialEq+Clone 자동)
#[component]
fn Greeting(name: String) -> Element {
    rsx! { p { "{name}, 안녕!" } }
}

// 3. Context 소비: App이 준 테마를 꺼내 쓰고 바꿈
#[component]
fn ThemeReader() -> Element {
    let mut theme = use_context::<Signal<String>>();
    rsx! {
        p { "테마: {theme}" }
        button {
            onclick: move |_| {
                let next = if theme() == "light" { "dark" } else { "light" };
                *theme.write() = next.to_string();
            },
            "테마 전환"
        }
    }
}

// 4. RSX for/if: 루프와 조건은 컴포넌트/엘리먼트 직접 배치
#[component]
fn ListDemo() -> Element {
    let items = vec!["rsx", "signal", "props"];
    rsx! {
        for item in items.clone() {
            p { "{item}" }
        }
        if items.len() > 2 {
            p { "조건 통과: 3개 이상" }
        }
    }
}

// 5. 입력: 제어 컴포넌트 (signal ↔ input 양방향)
#[component]
fn TextInput() -> Element {
    let mut value = use_signal(String::new);
    rsx! {
        input {
            value,
            oninput: move |e| *value.write() = e.value(),
        }
        p { "입력값: {value}" }
        button { onclick: move |_| value.write().clear(), "지우기" }
    }
}

// 6. 토글: bool signal + 조건 렌더
#[component]
fn Toggle() -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        button { onclick: move |_| *show.write() = !show(), "보이기/숨기기" }
        if show() {
            p { "짜잔!" }
        }
    }
}

// 7. 이펙트: 렌더 뒤 부수효과 (읽은 signal이 바뀌면 재실행)
#[component]
fn EffectDemo() -> Element {
    let mut count = use_signal(|| 0);
    use_effect(move || {
        println!("effect: count = {}", count());
    });
    rsx! {
        button { onclick: move |_| *count.write() += 1, "count: {count}" }
    }
}

// 8. 비동기: use_resource (None=로딩, Some=완료, hot_dog DogView의 오프라인판)
#[component]
fn AsyncDemo() -> Element {
    let data = use_resource(|| async move { "됐음".to_string() });
    let text = match data.read().as_ref() {
        None => "로딩중".to_string(),
        Some(v) => format!("결과: {v}"),
    };
    rsx! {
        p { "{text}" }
    }
}
