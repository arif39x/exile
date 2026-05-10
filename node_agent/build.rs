fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("exile.workload.v1.WorkloadDefinition", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.ResourceRequirements", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.ResourceRequirements.OS", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.ResourceRequirements.Architecture", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.HealthCheck", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.HealthCheck.Type", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.LogFormat", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.WorkloadMetrics", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("exile.workload.v1.LogLine", "#[derive(serde::Serialize, serde::Deserialize)]")
        .build_server(false)
        .compile(
            &["../shared_protocol/registry.proto", "../shared_protocol/workload.proto"],
            &["../shared_protocol"],
        )?;
    Ok(())
}
