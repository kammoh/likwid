extern crate likwid_sys;
extern crate libc;

pub use likwid_sys::{CpuInfo, CpuTopology, AccessMode};

struct Topology {
    cpu_info: & 'static CpuInfo,
    cpu_topo: & 'static CpuTopology,
}

impl Topology {
    fn new () -> Result<Topology, i32> {
        unsafe{
            let err = likwid_sys::topology_init();
            if err < 0 {
                Err(err)
            } else {
                let cpu_info = unsafe {
                    &*likwid_sys::get_cpuInfo()
                };
                Ok(Topology{
                    cpu_info,
                    cpu_topo: &*likwid_sys::get_cpuTopology(),
                })
            }
        }
    }
}

impl Drop for Topology {
    fn drop(&mut self) {
        unsafe {
            likwid_sys::topology_finalize();
        }
        println!("Topology is being dropped");
    }
}

struct Affinity {

}

impl Affinity {
    fn new() -> Affinity {
        Affinity{}
    }
}

struct Perfmon {

}

impl Perfmon {
    fn new() -> Result<Perfmon, i32> {
        unsafe{
        likwid_sys::HPMmode(AccessMode::ACCESSMODE_DAEMON);
        let err = perfmon_init(topo->numHWThreads, cpus);
        if err < 0 {
            Err(err)
        } else {
            Ok(Perfmon{})
        }
    }
}

impl Drop for Perfmon {
    fn drop(&mut self) {
        unsafe {
            likwid_sys::perfmon_finalize();
        }
        println!("Perfmon is being dropped");
    }
}


pub fn topology_init() -> Result<(), i32> {
    unsafe {
        let err = likwid_sys::topology_init();
        if err < 0 {
            Err(err)
        } else {
            Ok(())
        }
    }
}

pub fn print_supported_cpus() {
    unsafe {
        likwid_sys::print_supportedCPUs();
    }
}

#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn it_works() {
        
    }
}
