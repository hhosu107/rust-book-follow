Comments

- `panic!`, `continue`,  `loop` 등은 발산하는 타입을 가진다.
  - `!` type (diverging type).
  - 이 타입은 타입 검사를 스킵하는 타입이기 때문에 type 검사를 통과한다.
- `;`는 expression을 consume하고 () 값을 내뱉는 operator로 생각할 수 있다.
  - `A; B` 형태로 코드를 작성하면, A 값은 남지 않고 B 값만 남는다. 이는 `;`가
      consume했기 때문.
