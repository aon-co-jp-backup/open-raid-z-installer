//! OpenRaidZ Installer の画面。元のTypeScript(`main.ts`+`index.html`)を、
//! 同じ構成・同じTauriコマンドのままLeptos(CSR)で書き直したもの。

mod api;
mod i18n;
mod i18n_data;

use std::collections::BTreeMap;

use api::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

const RAID_LEVELS: [(&str, &str); 6] = [
    ("Raid0", "RAID0"),
    ("Raid1", "RAID1"),
    ("Raid5", "RAID5"),
    ("Raid6", "RAID6"),
    ("Z2", "RAID-Z2"),
    ("Z3", "RAID-Z3"),
];

fn main() {
    // panic時にブラウザのコンソールへ出す(外部クレートを増やさない最小の設定)。
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&format!("{info}").into());
    }));
    i18n::apply_direction(&i18n::load_lang());
    mount_to_body(App);
}

fn status_key(status: &str) -> String {
    format!("status_{status}")
}

#[component]
fn App() -> impl IntoView {
    let lang = RwSignal::new(i18n::load_lang());
    let lang2 = RwSignal::new(i18n::load_lang2());
    let hybrid = RwSignal::new(i18n::load_hybrid());
    // 現在の言語設定で訳す(キーは動的でもよい)。
    let tr = move |key: &str| i18n::t_display(key, &lang.get(), &lang2.get(), hybrid.get());
    let tr_now = move |key: &str| i18n::t_display(key, &lang.get_untracked(), &lang2.get_untracked(), hybrid.get_untracked());

    // ハードウェア・助言
    let accel_text = RwSignal::new(String::from("-"));
    let disks = RwSignal::new(Vec::<DiskInfo>::new());
    let advice = RwSignal::new(Vec::<Advice>::new());
    // 選択中ディスク(パス→サイズ)
    let selected = RwSignal::new(BTreeMap::<String, u64>::new());
    let confirmed = RwSignal::new(false);
    let loaded_once = RwSignal::new(false);

    // 対応状況パネル
    let status_open = RwSignal::new(false);
    let status = RwSignal::new(None::<SystemStatus>);
    // None=未実行 / Some(None)=実行中 / Some(Some(..))=結果
    let bench = RwSignal::new(None::<Option<Vec<BenchmarkEntry>>>);

    // フォーム
    let apply_level = RwSignal::new(String::from("Raid0"));
    let apply_name = RwSignal::new(String::from("tank"));
    let apply_result = RwSignal::new(String::new());
    let raid_level = RwSignal::new(String::from("Raid0"));
    let disk_count = RwSignal::new(String::from("4"));
    let mirror_width = RwSignal::new(String::from("2"));
    let raid_name = RwSignal::new(String::from("tank"));
    let raid_result = RwSignal::new(String::new());

    let load_hardware = move || {
        accel_text.set(tr_now("loading"));
        spawn_local(async move {
            let accel = invoke::<_, AcceleratorInfo>("detect_accelerator", &NoArgs {}).await;
            let d = invoke::<_, Vec<DiskInfo>>("list_physical_disks", &NoArgs {}).await;
            let a = invoke::<_, Vec<Advice>>("get_disk_advice", &NoArgs {}).await;
            match accel {
                Ok(x) => accel_text.set(format!("{}: {}", x.kind, x.description)),
                Err(e) => accel_text.set(e),
            }
            let d = d.unwrap_or_default();
            selected.update(|s| s.retain(|p, _| d.iter().any(|x| &x.path == p)));
            disks.set(d);
            advice.set(a.unwrap_or_default());
            loaded_once.set(true);
        });
    };
    // 起動時に1回だけ読み込む。
    load_hardware();

    let open_status = move |_| {
        status_open.set(true);
        spawn_local(async move {
            if let Ok(s) = invoke::<_, SystemStatus>("get_system_status", &NoArgs {}).await {
                status.set(Some(s));
            }
        });
    };
    let run_bench = move |_| {
        bench.set(Some(None));
        spawn_local(async move {
            let r = invoke::<_, Vec<BenchmarkEntry>>("benchmark_accelerators", &NoArgs {})
                .await
                .unwrap_or_default();
            bench.set(Some(Some(r)));
        });
    };

    let submit_raid = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let level = raid_level.get_untracked();
        let n: u32 = disk_count.get_untracked().trim().parse().unwrap_or(0);
        let width: u32 = mirror_width.get_untracked().trim().parse().unwrap_or(0);
        let name = raid_name.get_untracked();
        raid_result.set(tr_now("loading"));
        let (ok_label, err_label) = (tr_now("result_label"), tr_now("error_label"));
        spawn_local(async move {
            let res = if level == "Raid10" {
                invoke::<_, serde_json::Value>(
                    "init_raid10_preview",
                    &serde_json::json!({"req": {"disk_count": n, "mirror_width": width, "dataset_name": name}}),
                )
                .await
            } else {
                invoke::<_, serde_json::Value>(
                    "init_zpool_preview",
                    &serde_json::json!({"req": {"disk_count": n, "level": level, "dataset_name": name}}),
                )
                .await
            };
            raid_result.set(match res {
                Ok(v) => format!("{ok_label}: {v}"),
                Err(e) => format!("{err_label}: {e}"),
            });
        });
    };

    let submit_apply = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        apply_result.set(tr_now("loading"));
        let (ok_label, err_label) = (tr_now("result_label"), tr_now("error_label"));
        let picked: Vec<DiskSelection> = selected
            .get_untracked()
            .into_iter()
            .map(|(path, size_bytes)| DiskSelection { path, size_bytes })
            .collect();
        let req = serde_json::json!({"req": {
            "disks": picked,
            "level": apply_level.get_untracked(),
            "dataset_name": apply_name.get_untracked(),
            "confirm_data_loss": confirmed.get_untracked(),
        }});
        spawn_local(async move {
            let res = invoke::<_, serde_json::Value>("init_zpool_apply", &req).await;
            apply_result.set(match res {
                Ok(v) => format!("{ok_label}: {v}"),
                Err(e) => format!("{err_label}: {e}"),
            });
        });
    };

    let apply_disabled = move || !(confirmed.get() && !selected.get().is_empty());

    view! {
        <main class="container">
            <header class="app_header">
                <h1>{move || tr("app_title")}</h1>
                <div class="header_actions">
                    <button type="button" on:click=open_status>{move || tr("system_status_button")}</button>
                    <label class="lang_switcher">
                        <span>{move || tr("language_label")}</span>
                        <select on:change=move |ev| {
                            let v = event_target_value(&ev);
                            i18n::save_lang(&v);
                            lang.set(v);
                        }>
                            {i18n::LANGS.iter().map(|(code, name)| view! {
                                <option value=*code selected=move || lang.get() == *code>{*name}</option>
                            }).collect_view()}
                        </select>
                    </label>
                    <label class="lang_switcher">
                        <span>{move || tr("second_language_label")}</span>
                        <select
                            disabled=move || !hybrid.get()
                            on:change=move |ev| {
                                let v = event_target_value(&ev);
                                i18n::save_lang2(&v);
                                lang2.set(v);
                            }
                        >
                            {i18n::LANGS.iter().map(|(code, name)| view! {
                                <option value=*code selected=move || lang2.get() == *code>{*name}</option>
                            }).collect_view()}
                        </select>
                    </label>
                    <label class="checkbox_row">
                        <input
                            type="checkbox"
                            prop:checked=move || hybrid.get()
                            on:change=move |ev| {
                                let on = event_target_checked(&ev);
                                i18n::save_hybrid(on);
                                hybrid.set(on);
                            }
                        />
                        <span>{move || tr("hybrid_toggle_label")}</span>
                    </label>
                </div>
            </header>

            <section class="panel overlay_panel" hidden=move || !status_open.get()>
                <div class="overlay_panel_header">
                    <h2>{move || tr("system_status_title")}</h2>
                    <button type="button" on:click=move |_| status_open.set(false)>{move || tr("close_button")}</button>
                </div>
                <h3>{move || tr("system_status_os")}</h3>
                <p>{move || status.get().map(|s| s.current_os).unwrap_or_else(|| "-".to_string())}</p>
                <h3>{move || tr("system_status_os_compat")}</h3>
                <ul>
                    {move || status.get().map(|s| s.os_compatibility).unwrap_or_default().into_iter().map(|e| {
                        let label = tr(&status_key(&e.status));
                        view! { <li class=status_key(&e.status)><strong>{e.os}</strong>{format!(": {label} — {}", e.note)}</li> }
                    }).collect_view()}
                </ul>
                <h3>{move || tr("system_status_gpu")}</h3>
                <ul>
                    {move || {
                        let accs = status.get().map(|s| s.accelerators).unwrap_or_default();
                        if accs.is_empty() {
                            view! { <li>{tr("no_gpu_detected")}</li> }.into_any()
                        } else {
                            accs.into_iter()
                                .map(|a| view! { <li>{format!("[{}] {}: {}", a.vendor, a.kind, a.description)}</li> })
                                .collect_view()
                                .into_any()
                        }
                    }}
                </ul>
                <h3>{move || tr("system_status_storage")}</h3>
                <ul>
                    {move || {
                        let ds = status.get().map(|s| s.disks).unwrap_or_default();
                        if ds.is_empty() {
                            view! { <li>{tr("no_storage_detected")}</li> }.into_any()
                        } else {
                            ds.into_iter()
                                .map(|d| view! { <li>{format!("[{}] {} — {} GiB", d.media_type, d.path, gib(d.size_bytes))}</li> })
                                .collect_view()
                                .into_any()
                        }
                    }}
                </ul>
                <h3>{move || tr("benchmark_title")}</h3>
                <button type="button" on:click=run_bench>{move || tr("benchmark_run_button")}</button>
                <ul>
                    {move || match bench.get() {
                        None => ().into_any(),
                        Some(None) => view! { <li>{tr("loading")}</li> }.into_any(),
                        Some(Some(rows)) => rows
                            .into_iter()
                            .map(|e| view! { <li>{format!("{}: {:.1} MB/s", e.label, e.throughput_mb_per_sec)}</li> })
                            .collect_view()
                            .into_any(),
                    }}
                </ul>
            </section>

            <section class="panel">
                <h2>{move || tr("section_hardware")}</h2>
                <p><strong>{move || tr("accelerator_label")}</strong>": "<span>{move || accel_text.get()}</span></p>
                <p><strong>{move || tr("disks_label")}</strong></p>
                <ul>
                    {move || disks.get().into_iter().map(|d| {
                        let path = d.path.clone();
                        let path2 = d.path.clone();
                        let size = d.size_bytes;
                        view! {
                            <li><label>
                                <input
                                    type="checkbox"
                                    class="disk_select_checkbox"
                                    prop:checked=move || selected.get().contains_key(&path)
                                    on:change=move |ev| {
                                        let on = event_target_checked(&ev);
                                        let p = path2.clone();
                                        selected.update(|s| {
                                            if on { s.insert(p, size); } else { s.remove(&p); }
                                        });
                                    }
                                />
                                {format!(" {} — {} GiB", d.path, gib(d.size_bytes))}
                            </label></li>
                        }
                    }).collect_view()}
                </ul>
                <p class="warning" hidden=move || !disks.get().is_empty() || !loaded_once.get()>{move || tr("no_disks_admin_warning")}</p>
                <button on:click=move |_| load_hardware()>{move || tr("refresh_button")}</button>
            </section>

            <section class="panel">
                <h2>{move || tr("section_apply")}</h2>
                <p class="warning">{move || tr("apply_warning")}</p>
                <form class="row" on:submit=submit_apply>
                    <label>
                        <span>{move || tr("raid_level_label")}</span>
                        <select on:change=move |ev| apply_level.set(event_target_value(&ev))>
                            {RAID_LEVELS.iter().map(|(v, n)| view! { <option value=*v>{*n}</option> }).collect_view()}
                        </select>
                    </label>
                    <label>
                        <span>{move || tr("dataset_name_label")}</span>
                        <input type="text" prop:value=move || apply_name.get() on:input=move |ev| apply_name.set(event_target_value(&ev)) />
                    </label>
                    <label class="checkbox_row">
                        <input type="checkbox" prop:checked=move || confirmed.get() on:change=move |ev| confirmed.set(event_target_checked(&ev)) />
                        <span>{move || tr("confirm_data_loss_label")}</span>
                    </label>
                    <button type="submit" disabled=apply_disabled>{move || tr("apply_button")}</button>
                </form>
                <div>{move || apply_result.get()}</div>
            </section>

            <section class="panel">
                <h2>{move || tr("section_advice")}</h2>
                <ul>
                    {move || advice.get().into_iter().map(|a| view! {
                        <li class=format!("advice_{}", a.severity.to_lowercase())><strong>{a.title}</strong>{format!(": {}", a.detail)}</li>
                    }).collect_view()}
                </ul>
            </section>

            <section class="panel">
                <h2>{move || tr("section_raid")}</h2>
                <form class="row" on:submit=submit_raid>
                    <label>
                        <span>{move || tr("raid_level_label")}</span>
                        <select on:change=move |ev| raid_level.set(event_target_value(&ev))>
                            {RAID_LEVELS.iter().map(|(v, n)| view! { <option value=*v>{*n}</option> }).collect_view()}
                            <option value="Raid10">"RAID10"</option>
                        </select>
                    </label>
                    <label>
                        <span>{move || tr("disk_count_label")}</span>
                        <input type="number" min="1" prop:value=move || disk_count.get() on:input=move |ev| disk_count.set(event_target_value(&ev)) />
                    </label>
                    <label style:display=move || if raid_level.get() == "Raid10" { "" } else { "none" }>
                        <span>{move || tr("mirror_width_label")}</span>
                        <input type="number" min="2" prop:value=move || mirror_width.get() on:input=move |ev| mirror_width.set(event_target_value(&ev)) />
                    </label>
                    <label>
                        <span>{move || tr("dataset_name_label")}</span>
                        <input type="text" prop:value=move || raid_name.get() on:input=move |ev| raid_name.set(event_target_value(&ev)) />
                    </label>
                    <button type="submit">{move || tr("init_button")}</button>
                </form>
                <div>{move || raid_result.get()}</div>
            </section>
        </main>
    }
}
