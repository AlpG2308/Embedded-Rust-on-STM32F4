/*Sensor Abstraction layer
 Create a Peripheral API trait for our sensorAbstraction
 + Initalize Sensor
 + perform distance measurement
 + read raw data from register address
 ---Error Handling---
 Use enums to cover all possible errors
I2C,
Timer,
Data,
generic -> all other errors,
---Data Container for raw measurement---
struct
t in us
photons (from avalance photodiode)
signal rate (for resolution calculation)
---Low Level Controller Struct---
struct Sensor with i2c trait
i2c init as embassy i2c device as static mitigate borrowing issues
registeradress

---Helpers---
async read and write func
---Implement trait for our sensor VL53L0x
1. implement generic embassy i2c as async and static -> check Liftime
2. init sensor at start (wait for calibration) -> make sure peripheral is running (budget startup)
3. read distance
4. read raw
---High level sampler---
struct of Trait <Sensor>
-> new
-> sample certain amount 'steps'
-> push to vec with capacity
-> wait make sure sampling is done
*/
pub trait TimeOfFlight {
    //init sensor
    async fn init(&mut self) -> Result<(), SensorError>;
    // Perform distnace measurement---
    async fn read_distance(&mut self) -> Result<u16, SensorError>;
    // Raw Photondata
    async fn read_raw(&mut self) -> Result<RawMeasurement, SensorError>;
}

#[derive(Debug)]
pub enum SensorError {
    I2C,
    responseTimeout,
    InvalidData,
    Other,
}

#[derive(Debug, Clone)]
pub struct RawMeasurement {
    pub time_us: u64,
    pub photons: u16,
    pub signal_rate: f32,
}

pub struct VL53L0x<I2c> {
    i2c: I2cDevice<'static, I2c>,
    address: u8,
}
//check embassyst32 crate
async fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), SensorError> {
    self.i2c
        .write(self.address, &[reg, value])
        .await
        .map_err(|_| SensorError::I2C)
}

async fn read_reg(&mut self, reg: u8) -> Result<u8, SensorError> {
    let mut buf = [0u8; 1];
    self.i2c
        .write_read(self.address, &[reg], &mut buf)
        .await
        .map_err(|_| SensorError::I2C)?;
    Ok(buf[0])
}

impl<I2C> TimeOfFlight for VL53L0x<I2C>
where
    I2C: embedded_hal_async::i2c::I2c + 'static,
{
    async fn init(&mut self) -> Result<(), SensorError> {
        // TODO: Check init seq in datasheet
        //
    }
    async fn read_distance(&mut self) -> Result<u16, SensorError> {
        //TODO: Check datasheet implement distance measurement
        //
    }
    async fn read_raw(&mut self) -> Result<u16, SensorError> {
        //TODO: Check registers for raw measurement
        //
        //
    }
}

//TODO:Implement High Level call/sampler
pub struct ToFSampler<T: TimeofFlight> {
    sensor: T,
}
