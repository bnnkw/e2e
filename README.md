# E2E

YAML-based E2E testing tool.

Requires a WebDriver binary. Currently only Microsoft Edge is supported.

## Example `e2e.yaml`

```yaml
driver:
  host: localhost
  port: 4444
  headless: true
  window: { x: 0, y: 0, width: 1920, height: 1080 }

vars:
  base_url: "http://localhost:8080"
  username: admin

tasks:
  login:
    arg_names: [username, password]
    steps:
      - !goto "{base_url}/login"
      - !send_keys { selector: "#username", value: "{username}" }
      - !send_keys { selector: "#password", value: "{password}" }
      - !click "#login-button"

scenarios:
  login_success:
    name: "Successful Login"
    steps:
      - !task_run { id: login, args: ["{username}", "secret"] }
      - !wait_displayed { selector: "#dashboard", timeout: 5000, interval: 500 }
      - !assert_eq { expected: "Welcome!", target: !element { selector: ".msg", attr: text } }
```

## Usage

```bash
e2e run                     # run all scenarios
e2e run login_success       # run specific scenario
e2e -f my_tests.yaml run    # use a different config file
e2e config                  # print parsed config
```

## Steps

| Step | Description |
|------|-------------|
| `!goto <URL>` | Navigate to URL |
| `!click <SELECTOR>` | Click element |
| `!focus <SELECTOR>` | Focus element |
| `!send_keys { selector, value }` | Clear and type text into element |
| `!select { selector, kind, value }` | Select option by `value` or `text` |
| `!screen_shot <PATH>` | Save screenshot to path |
| `!wait_displayed { selector, timeout, interval }` | Wait until element is displayed (ms) |
| `!click_and_wait { selector, timeout, interval }` | Click and wait for navigation (ms) |
| `!wait <MS>` | Wait for specified milliseconds |
| `!accept_alert` | Accept browser alert |
| `!upload_file { selector, file }` | Upload file to input element |
| `!switch_to_window { index, maximum }` | Switch to browser window by index |
| `!task_run { id, args }` | Run a reusable task |
| `!assert_eq { expected, target }` | Assert value matches expected. Target: `url` or `!element { selector, attr }` (attr: `text`, `id`, `class`) |

Variables from `vars` or task `arg_names` are expanded with `{name}`. Escape literal braces: `{{`, `}}`.
