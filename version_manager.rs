mod preloaded; // do not modify this!
use preloaded::VMError;

struct VersionManager {
    major: i32,
    minor: i32,
    patch: i32,
    history: Vec<String>,
}

impl VersionManager {
    fn new() -> Self {
        VersionManager {
            major: 0,
            minor: 0,
            patch: 1,
            history: vec![],
        }
    }

    fn from_version(s: &str) -> Result<Self, VMError> {
        let a = (s.to_owned() + ".0.0.1")
            .trim_start_matches(".")
            .split(".")
            .take(3)
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        if a.iter().any(|e| e.parse::<i32>().unwrap_or(-1) == -1) {
            return Err(VMError::InvalidVersion);
        }
        let v = a
            .iter()
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut vm = VersionManager::new();
        vm.major = v[0];
        vm.minor = v[1];
        vm.patch = v[2];
        Ok(vm)
    }

    fn major(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
        self
    }

    fn minor(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.minor += 1;
        self.patch = 0;
        self
    }

    fn patch(&mut self) -> &mut Self {
        self.history.push(self.release());
        self.patch += 1;
        self
    }

    fn rollback(&mut self) -> Result<&mut Self, VMError> {
        if self.history.len() < 1 {
            return Err(VMError::NoHistory);
        }
        let s = &self.history.pop().unwrap();
        let mut vm = VersionManager::from_version(s).unwrap();
        self.major = vm.major;
        self.minor = vm.minor;
        self.patch = vm.patch;
        Ok(self)
    }

    fn release(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
