use eyre::eyre;
use slurm_spank::{spank_log_user, Context, Plugin, SpankHandle, SpankOption, SPANK_PLUGIN};
use std::convert::TryFrom;
use std::error::Error;
use tracing::error;

SPANK_PLUGIN!(b"tests\0", 0x160502, SpankTest);

#[derive(Default)]
struct SpankTest {}

impl Plugin for SpankTest {
    fn init(&mut self, spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        let context = spank.context()?;

        let usage = match spank.context()? {
            Context::Local => "Run selected test (srun)",
            Context::Allocator => "Run selected test (salloc/sbatch)",
            _ => "Run selected test",
        };

        spank.register_option(SpankOption::new("test").takes_value("test").usage(usage))?;

        if context != Context::Slurmd {
            error!("Plugin arguments {}", spank.plugin_argv()?.join(","));
        }

        // Check that we return None in invalid contexts
        assert!(spank.get_option_value("test")?.is_none());
        Ok(())
    }
    fn init_post_opt(&mut self, spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        let Some(test) = spank.get_option_value("test")? else {
            return Ok(());
        };

        spank_log_user!("{:?}: selected test: {test}", spank.context()?);

        if test == "client-error"
            && (spank.context()? == slurm_spank::Context::Local
                || spank.context()? == slurm_spank::Context::Allocator)
        {
            return Err(eyre!("Expected an error").into());
        }

        if test == "remote-error" && (spank.context()? == slurm_spank::Context::Remote) {
            return Err(eyre!("Expected an error").into());
        }

        if test == "values" && spank.context()? == slurm_spank::Context::Remote {
            spank_log_user!("spank_remote_job_id: {}", spank.job_id()?);
            spank_log_user!("spank_remote_job_ncpus: {}", spank.job_ncpus()?);
            spank_log_user!("spank_remote_job_nnodes: {}", spank.job_nnodes()?);
            spank_log_user!("spank_remote_job_nodeid: {}", spank.job_nodeid()?);
            spank_log_user!("spank_remote_job_stepid: {}", spank.job_stepid()?);
            spank_log_user!("spank_remote_job_alloc_cores: {}", spank.job_alloc_cores()?);
            spank_log_user!("spank_remote_job_alloc_mem: {}", spank.job_alloc_mem()?);
            spank_log_user!(
                "spank_remote_job_total_task_count: {}",
                spank.job_total_task_count()?
            );
            spank_log_user!(
                "spank_remote_job_local_task_count: {}",
                spank.job_local_task_count()?
            );
            spank_log_user!("spank_remote_job_argv: {}", spank.job_argv()?.join(","));

            spank_log_user!("spank_remote_step_alloc_mem: {}", spank.step_alloc_mem()?);
            spank_log_user!(
                "spank_remote_step_alloc_cores: {}",
                spank.step_alloc_cores()?
            );
            spank_log_user!(
                "spank_remote_step_cpus_per_task: {}",
                spank.step_cpus_per_task()?
            );
            spank_log_user!("spank_remote_job_gid: {}", spank.job_gid()?);
            spank_log_user!("spank_remote_job_uid: {}", spank.job_uid()?);
            spank_log_user!(
                "spank_remote_job_supplementary_gids: {}",
                spank
                    .job_supplementary_gids()?
                    .iter()
                    .map(|g| g.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
        Ok(())
    }

    fn task_post_fork(&mut self, spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        let Some(test) = spank.get_option_value("test")? else {
            return Ok(());
        };

        if test == "values" {
            let id = spank.task_global_id()?;
            if id == 12 {
                let pid = spank.task_pid()?;
                let local_id = spank.task_id()?;

                spank_log_user!("spank_task_global_id: {}", spank.task_global_id()?);
                spank_log_user!("spank_task_id: {}", local_id);
                spank_log_user!("spank_task_pid: {}", pid);
                spank_log_user!("spank_id_from_pid: {}", spank.pid_to_local_id(pid)?);
                spank_log_user!("spank_global_id_from_pid: {}", spank.pid_to_global_id(pid)?);
                spank_log_user!(
                    "spank_local_id_from_global: {}",
                    spank.global_to_local_id(id)?
                );
                spank_log_user!(
                    "spank_global_id_from_local: {}",
                    spank.local_to_global_id(u32::try_from(local_id)?)?
                );
            }
        }
        Ok(())
    }

    fn job_prolog(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn local_user_init(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn user_init(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn task_init_privileged(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn task_init(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn task_exit(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn job_epilog(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn slurmd_exit(&mut self, spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        // Check that we return None in invalid contexts
        assert!(spank.get_option_value("test")?.is_none());
        Ok(())
    }

    fn exit(&mut self, _spank: &mut SpankHandle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
