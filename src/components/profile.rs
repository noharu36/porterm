use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class="profile-container">
            <div class="profile-image-container">
                <img src="assets/icon.jpeg" class="profile-image" alt="Profile Avatar" />
            </div>
            <div class="info-list">
                <div class="info-item" style="border: none; margin-bottom: 5px;">
                    <span class="value" style="color: var(--fg); font-weight: bold;">"harukun@myhostname"</span>
                </div>
                <div class="info-item" style="border: none; margin-bottom: 5px;">
                     <span class="value">"-----------"</span>
                </div>

                <div class="info-item">
                    <span class="key">"OS"</span>
                    <span class="value">"macOS Tahoe 26.2 arm64"</span>
                </div>
                <div class="info-item">
                    <span class="key">"Host"</span>
                    <span class="value">"MacBook Pro (14-inch, Nov 2023, Two Thunderbolt / USB 4 ports)"</span>
                </div>
                <div class="info-item">
                    <span class="key">"Kernel"</span>
                    <span class="value">"Darwin 25.2.0"</span>
                </div>
                <div class="info-item">
                    <span class="key">"Shell"</span>
                    <span class="value">"zsh 5.9"</span>
                </div>
                <div class="info-item">
                    <span class="key">"WM"</span>
                    <span class="value">"aerospace"</span>
                </div>
                <div class="info-item">
                    <span class="key">"Theme"</span>
                    <span class="value">"Gruvbox Dark"</span>
                </div>
                 <div class="info-item">
                    <span class="key">"Terminal"</span>
                    <span class="value">"rio 0.2.37"</span>
                </div>
            </div>
        </div>
    }
}
