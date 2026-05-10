pub async fn login_play(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::smoke::run(host).await?;
    println!("login-play probe ok");
    Ok(())
}

pub async fn multiplayer_mutation(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::multiplayer_mutation::run(host).await?;
    println!("multiplayer-mutation probe ok");
    Ok(())
}

pub async fn movement_authority(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::movement_authority::run(host).await?;
    println!("movement-authority probe ok");
    Ok(())
}

pub async fn persist_place(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::persistence::place(host).await?;
    println!("persist-place probe ok");
    Ok(())
}

pub async fn persist_check(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::persistence::check(host).await?;
    println!("persist-check probe ok");
    Ok(())
}

pub async fn storage_section_persistence(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::storage_section_persistence::run(host).await?;
    println!("storage-section-persistence probe ok");
    Ok(())
}

pub async fn profile_reconnect(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::profile_reconnect::run(host).await?;
    println!("profile-reconnect probe ok");
    Ok(())
}

pub async fn chunk_stream(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::chunk_stream::run(host).await?;
    println!("chunk-stream probe ok");
    Ok(())
}

pub async fn terrain_generation(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::terrain_generation::run(host).await?;
    println!("terrain-generation probe ok");
    Ok(())
}

pub async fn terrain_rivers(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::terrain_rivers::run(host).await?;
    println!("terrain-rivers probe ok");
    Ok(())
}

pub async fn terrain_caves(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::terrain_caves::run(host).await?;
    println!("terrain-caves probe ok");
    Ok(())
}

pub async fn terrain_quality(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::terrain_quality::run(host).await?;
    println!("terrain-quality probe ok");
    Ok(())
}

pub async fn scale_chunk_stream(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::scale_chunk_stream::run(host).await?;
    println!("scale-chunk-stream probe ok");
    Ok(())
}

pub async fn scale_load_metrics(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::scale_load_metrics::run(host).await?;
    println!("scale-load-metrics probe ok");
    Ok(())
}

pub async fn scale_moving_pending(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::scale_moving_pending::run(host).await?;
    println!("scale-moving-pending probe ok");
    Ok(())
}

pub async fn render_distance(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::render_distance::run(host).await?;
    println!("render-distance probe ok");
    Ok(())
}

pub async fn render_moving_pending(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::render_moving_pending::run(host).await?;
    println!("render-moving-pending probe ok");
    Ok(())
}

pub async fn survival_item(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::survival_item::run(host).await?;
    println!("survival-item probe ok");
    Ok(())
}

pub async fn survival_vitals(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::survival_vitals::run(host).await?;
    println!("survival-vitals probe ok");
    Ok(())
}

pub async fn inventory_sync(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::inventory_sync::run(host).await?;
    println!("inventory-sync probe ok");
    Ok(())
}

pub async fn item_pickup(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::item_pickup::run(host).await?;
    println!("item-pickup probe ok");
    Ok(())
}

pub async fn smp_commands(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::smp_commands::run(host).await?;
    println!("smp-commands probe ok");
    Ok(())
}

pub async fn online_auth(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    super::online_auth_probe::run(host).await?;
    println!("online-auth probe ok");
    Ok(())
}
