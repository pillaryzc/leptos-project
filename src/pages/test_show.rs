#[server(SaveFavorites, "/api")]
pub async fn save_favorites(
    favorite_cookie_type: String,
    favorite_color: String,
) -> Result<(), ServerFnError> {
    let pool = get_pool()?;

    let query = "
        INSERT INTO COOKIES 
        (favorite_cookie_type, favorite_color)
        VALUES ($1, $2)
    ";

    sqlx::query(query)
        .bind(favorite_cookie_type)
        .bind(favorite_color)
        .execute(&pool)
        .await
        .map_err(|e| 
            ServerFnError::ServerError(e.to_string())?;

    Ok(format!("Here, have some {favorite_color} {favorite_cookie_type} cookies!"))
}

#[component]
pub fn FavoritesForm() -> impl IntoView {
    let favorites = create_server_action::<SaveFavorites>();
    let value = favorites.value();
    view! { 
        <ActionForm action=favorites>
            <label>
                "Favorite type of cookie"
                <input
                    type="text"
                    name="favorite_cookie_type"
                />
            </label>
            <label>
                "Favorite color"
                <input
                    type="text"
                    name="favorite_color"
                />
            </label>
            <input type="submit"/>
        </ActionForm>
        <Show when=favorites.pending()>
            <div>"Loading..."</div>
        </Show>
        <Show when=move || value.with(Option::is_some)>
            <div>{value}</div>
        </Show>
    }
}