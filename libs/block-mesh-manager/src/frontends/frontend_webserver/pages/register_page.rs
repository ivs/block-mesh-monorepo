use leptos::*;
use leptos_router::A;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="bg-dark-blue h-screen">
            <form action="/register" method="post">
                <div class="bg-dark-blue flex justify-center items-center h-screen">
                    <div class="bg-dark-blue border-off-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                        <h2 class="font-bebas-neue text-off-white text-2xl font-semibold text-center mb-6">
                            Register
                        </h2>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                                for="email"
                            >
                                Email
                            </label>
                            <input
                                class="text-black shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="text"
                                id="email"
                                placeholder="Email"
                                name="email"
                                required
                            />
                        </div>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                                for="password"
                            >
                                Password
                            </label>
                            <input
                                class="text-black shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="password"
                                name="password"
                                placeholder="******************"
                                required
                            />
                        </div>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                                for="password_confirm"
                            >
                                Confirm
                                Password
                            </label>
                            <input
                                class="text-black shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="password_confirm"
                                name="password_confirm"
                                placeholder="******************"
                                required
                            />
                        </div>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                                for="invite_code"
                            >
                                Invite Code
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="text"
                                id="invite_code"
                                name="invite_code"
                                placeholder="Invite Code"
                            />
                        </div>
                        <div class="flex items-center justify-between">
                            <button
                                class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                                type="submit"
                            >
                                Submit
                            </button>
                            <A
                                class="font-open-sans mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:text-cyan"
                                href="/ui/login"
                            >
                                Login
                            </A>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    }
}
