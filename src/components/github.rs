use chrono::{DateTime, Duration, Local, Utc};
use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Actor {
    pub display_login: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: Actor,
    pub repo: Repo,
    pub created_at: DateTime<Utc>,
}

async fn fetch_events() -> Result<Vec<GithubEvent>, String> {
    // Increase per_page to get more history for the chart
    let url = "https://api.github.com/users/noharu36/events/public?per_page=100";
    let resp = Request::get(url).send().await.map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(format!("API Error: {}", resp.status()));
    }

    let events: Vec<GithubEvent> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(events)
}

#[component]
pub fn GithubActivity() -> impl IntoView {
    let events_resource = create_resource(|| (), |_| fetch_events());

    view! {
        <div class="github-activity">
            <Suspense fallback=move || view! { <p>"Loading activity..."</p> }>
                {move || {
                    events_resource.get().map(|result| match result {
                        Ok(events) => {
                             // --- Chart Calculation Logic ---
                            let today = Local::now().date_naive();
                            let start_date = today - Duration::days(29); // 30 days total including today

                            // Bucket events by date
                            let mut daily_counts: HashMap<chrono::NaiveDate, i32> = HashMap::new();
                            for event in &events {
                                // Convert UTC to Local date
                                let date = event.created_at.with_timezone(&Local).date_naive();

                                if date >= start_date && date <= today {
                                    *daily_counts.entry(date).or_insert(0) += 1;
                                }
                            }

                            // Generate data points for SVG
                            // SVG ViewBox: 0 0 600 150
                            // Margins: Left 40, Bottom 20, Top 10, Right 10
                            // Drawing Area: X=40..600 (width 560), Y=10..130 (height 120)

                            let max_commits = daily_counts.values().max().cloned().unwrap_or(0).max(5) as f32; // Min scale 5
                            let height_scale = 120.0 / max_commits;

                            let mut points_str = String::new();
                            let mut circles_view = Vec::new();
                            let mut x_labels_view = Vec::new();
                            let mut y_labels_view = Vec::new();

                            // Y-axis labels (0, max/2, max)
                            y_labels_view.push(view! { <text x="35" y="130" text-anchor="end" class="chart-label">"0"</text> });
                            y_labels_view.push(view! { <text x="35" y="70" text-anchor="end" class="chart-label">{(max_commits / 2.0).round().to_string()}</text> });
                            y_labels_view.push(view! { <text x="35" y="20" text-anchor="end" class="chart-label">{max_commits.to_string()}</text> });

                            for i in 0..30 {
                                let date = start_date + Duration::days(i);
                                let count = *daily_counts.get(&date).unwrap_or(&0);

                                let x = 40.0 + (i as f32 * (560.0 / 29.0));
                                let y = 130.0 - (count as f32 * height_scale);

                                if i == 0 {
                                    points_str.push_str(&format!("{},{}", x, y));
                                } else {
                                    points_str.push_str(&format!(" {},{}", x, y));
                                }

                                // Add circle for non-zero points
                                if count > 0 {
                                     circles_view.push(view! {
                                        <circle cx=x cy=y r="4" fill="var(--bg0)" stroke="var(--yellow)" stroke-width="2">
                                            <title>{format!("{}: {} commits", date.format("%Y-%m-%d"), count)}</title>
                                        </circle>
                                    });
                                }

                                // X-axis labels every 5 days
                                if i % 5 == 0 || i == 29 {
                                    x_labels_view.push(view! {
                                        <text x=x y="145" text-anchor="middle" class="chart-label">
                                            {date.format("%m/%d").to_string()}
                                        </text>
                                    });
                                }
                            }

                            view! {
                                <div class="chart-container" style="margin-bottom: 20px; padding: 10px; border-bottom: 1px solid var(--bg2);">
                                    <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 10px;">
                                        <h4 style="margin: 0; color: var(--fg4);">"Activity Pulse (Last 30 Days)"</h4>
                                        <span style="font-size: 0.8em; color: var(--gray);">
                                            {format!("Total: {}", daily_counts.values().sum::<i32>())}
                                        </span>
                                    </div>

                                    <svg width="100%" height="150" viewBox="0 0 600 150" style="background: var(--bg0-h); border-radius: 4px;">
                                        // Grid lines
                                        <line x1="40" y1="20" x2="600" y2="20" stroke="var(--bg2)" stroke-dasharray="4" />
                                        <line x1="40" y1="75" x2="600" y2="75" stroke="var(--bg2)" stroke-dasharray="4" />
                                        <line x1="40" y1="130" x2="600" y2="130" stroke="var(--fg4)" />
                                        <line x1="40" y1="10" x2="40" y2="130" stroke="var(--fg4)" />

                                        // Data Line
                                        <polyline
                                            points=points_str
                                            fill="none"
                                            stroke="var(--orange)"
                                            stroke-width="2"
                                            stroke-linejoin="round"
                                        />

                                        // Points
                                        {circles_view}

                                        // Labels
                                        {x_labels_view}
                                        {y_labels_view}
                                    </svg>
                                </div>

                                <h4 style="margin: 10px 0; color: var(--fg4);">"Recent Events"</h4>
                                <ul class="event-list">
                                    {events.into_iter().take(10).map(|event| {
                                        view! {
                                            <li class="event-item">
                                                <span class="event-time">
                                                    {event.created_at.format("%m-%d %H:%M").to_string()}
                                                </span>
                                                <span class="event-type" style="color: var(--purple)">
                                                    {format!("[{}] ", event.event_type.replace("Event", ""))}
                                                </span>
                                                <span class="event-repo" style="color: var(--blue)">
                                                    {event.repo.name}
                                                </span>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }.into_view()
                        }
                        Err(e) => view! { <p class="error">{format!("Error: {}", e)}</p> }.into_view(),
                    })
                }}
            </Suspense>
        </div>
    }
}
