use crate::policy::Policy;
use axum::response::Html;

/// GET /
pub async fn handler() -> Html<String> {
    static INDEX_HTML: &str = include_str!("../../templates/index.html");
    static TABLE_HTML: &str = include_str!("../../templates/policy_table.html");
    static ROW_HTML: &str = include_str!("../../templates/policy_row.html");

    let rows = render_policy_rows(ROW_HTML);
    let table = TABLE_HTML.replace("{{rows}}", &rows);
    let page = INDEX_HTML.replace("{{policy_table}}", &table);

    Html(page)
}

fn render_policy_rows(row_tpl: &str) -> String {
    let policies = [
        Policy::Observe,
        Policy::Prepare,
        Policy::Defend,
        Policy::Contain,
        Policy::Suppress,
    ];
    let mut out = String::new();
    for p in policies {
        out.push_str(
            &row_tpl
                .replace("{{name}}", &p.to_string())
                .replace("{{level}}", &p.level().to_string())
                .replace("{{summary}}", p.summary()),
        );
    }
    out
}
