# basics — Dioxus 기초 8개 한 파일에

`hot_dog` 다음 단계. 실행형 1개로 `signal / props / context / rsx for·if / input / toggle / effect / resource`만 익힌다.

실행:

```sh
dx serve
# 데스크탑으로: cargo run --features desktop --no-default-features
```

## 파일

- `src/main.rs` — 전부 (60줄). `Cargo.toml`은 `hot_dog`에서 `reqwest/serde` 뺀 것.

## 1. 실행 (`main` + `App`)

```rust
dioxus::launch(App);
```

`#[component]` 함수가 UI 단위. `App`이 최상위 조립.

## 2. 상태: `Counter` (`use_signal` + `use_memo`)

- `use_signal(|| 0)` — 컴포넌트 지역 상태. `count()`로 읽기, `*count.write() += 1`로 쓰기. 쓰면 읽은 곳만 리렌더.
- `use_memo(move || count() * 2)` — 파생값. `count`를 읽었으니 `count`가 바뀔 때만 재계산. 비싼 계산 캐싱용.

버튼 클릭 → `write()` → `h1` 다시 그림. 이게 전부.

## 3. 전달: `Greeting` (props)

```rust
#[component]
fn Greeting(name: String) -> Element
```

- props는 owned 값 (`String`, `Vec<T>`). `&str` 금지.
- `PartialEq + Clone` 필요 (매크로가 처리). 부모가 다른 `name`을 주면만 리렌더.

## 4. 공유: `ThemeReader` (Context API)

제공 (App):

```rust
use_context_provider(|| use_signal(|| "light".to_string()));
```

소비 (자식):

```rust
let mut theme = use_context::<Signal<String>>();
```

같은 타입 = 같은 방. `props`를 계층마다 넘기기 싫을 때 쓴다.

## 5. 나열·분기: `ListDemo` (RSX `for` / `if`)

- `for item in items { p { ... } }` — 루프 안에 엘리먼트 직접.
- `if cond { ... }` — 조건 안에 엘리먼트 직접.
- 이터레이터(`.map`) 쓸 때만 `{...}`로 감싼다. `for`가 있으면 `for`를 써라.

## 6. 입력: `TextInput` (제어 컴포넌트)

- `value`에 signal을 직접 꽂고 `oninput: move |e| *value.write() = e.value()`로 양방향 바인딩.
- 읽는 쪽(`p { "{value}" }`)은 타이핑마다 자동 리렌더. 지우기는 `value.write().clear()` 한 줄.

## 7. 토글: `Toggle` (bool + 조건)

- `use_signal(|| false)`, 클릭마다 `*show.write() = !show()`.
- `if show() { ... }` — 조건 안에 엘리먼트 직접. 모달·접기·탭의 최소형.

## 8. 이펙트: `EffectDemo` (`use_effect`)

- 렌더 뒤 실행. 클로저 안에서 읽은 signal이 바뀌면 재실행.
- `println!`, `localStorage`, 포커스 등 브라우저 부수효과는 여기. 첫 렌더 값을 서버와 똑같이 맞춰야 hydration이 안 깨진다.

## 9. 비동기: `AsyncDemo` (`use_resource`)

- `use_resource(|| async move { ... })`는 처음에 1번, 읽은 signal이 바뀌면 재실행.
- `None` = 로딩중, `Some(v)` = 완료. rsx 밖에서 `match`로 문자열을 골라 `p { "{text}" }` 한 줄로 그림 — `hot_dog`의 `DogView`와 같은 패턴, 네트워크 없이 익히기용.

## 다음에 볼 것 (지금은 스킵)

- 진짜 네트워크 `use_resource` → `hot_dog/src/main.rs`의 `DogView`가 이미 정답.
- 라우터·풀스택 → 필요해지면 `dioxus = { features = ["router"] }` 추가하고 그때 배운다.
