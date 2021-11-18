use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::soyboy::utils;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Parameter {
    // global parameter
    MasterVolume = 0,
    Pitch,
    OscillatorType,
    // envelope generator
    EgAttack,
    EgDecay,
    EgSustain,
    EgRelease,
    // square wave oscillator
    OscSqDuty,
    OscSqSweepType,
    OscSqSweepAmount,
    OscSqSweepPeriod,
    // noise oscillator
    OscNsInterval,
    // wavetable oscillator
    OscWtTableIndex,
    OscWtTableValue,
}

impl TryFrom<u32> for Parameter {
    type Error = ();

    fn try_from(id: u32) -> Result<Self, Self::Error> {
        if id == Parameter::MasterVolume as u32 {
            Ok(Parameter::MasterVolume)
        } else if id == Parameter::OscillatorType as u32 {
            Ok(Parameter::OscillatorType)
        } else if id == Parameter::Pitch as u32 {
            Ok(Parameter::Pitch)
        } else if id == Parameter::EgAttack as u32 {
            Ok(Parameter::EgAttack)
        } else if id == Parameter::EgDecay as u32 {
            Ok(Parameter::EgDecay)
        } else if id == Parameter::EgSustain as u32 {
            Ok(Parameter::EgSustain)
        } else if id == Parameter::EgRelease as u32 {
            Ok(Parameter::EgRelease)
        } else if id == Parameter::OscSqDuty as u32 {
            Ok(Parameter::OscSqDuty)
        } else if id == Parameter::OscSqSweepType as u32 {
            Ok(Parameter::OscSqSweepType)
        } else if id == Parameter::OscSqSweepAmount as u32 {
            Ok(Parameter::OscSqSweepAmount)
        } else if id == Parameter::OscSqSweepPeriod as u32 {
            Ok(Parameter::OscSqSweepPeriod)
        } else if id == Parameter::OscNsInterval as u32 {
            Ok(Parameter::OscNsInterval)
        } else if id == Parameter::OscWtTableIndex as u32 {
            Ok(Parameter::OscWtTableIndex)
        } else if id == Parameter::OscWtTableValue as u32 {
            Ok(Parameter::OscWtTableValue)
        } else {
            Err(())
        }
    }
}

pub struct ParamIter(u32);

impl ParamIter {
    pub fn new() -> ParamIter {
        ParamIter(Parameter::MasterVolume as u32)
    }
}

impl Iterator for ParamIter {
    type Item = Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(p) = Parameter::try_from(self.0) {
            self.0 += 1;
            Some(p)
        } else {
            None
        }
    }
}

impl Parameter {
    pub fn iter() -> ParamIter {
        ParamIter::new()
    }
}

pub trait Parametric<Parameter> {
    fn set_param(&mut self, param: &Parameter, value: f64);
    fn get_param(&self, param: &Parameter) -> f64;
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterType {
    NonLinear,
    Linear,
    List,
    Integer,
}

pub trait Normalizable<T> {
    fn denormalize(&self, normalized: f64) -> T;
    fn normalize(&self, plain: T) -> f64;
    fn format(&self, normalized: f64) -> String;
    fn parse(&self, string: &str) -> Option<f64>;
}

#[derive(Clone, Copy)]
pub struct NonLinearParameter {
    plain_zero: f64,
    plain_min: f64,
    plain_max: f64,
    plain_one: f64,
    factor: f64,
    diverge: bool,
}

impl Normalizable<f64> for NonLinearParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        if normalized == 0.0 {
            self.plain_zero
        } else if normalized == 1.0 {
            self.plain_one
        } else {
            let denormalizer = if self.diverge {
                utils::divergent_denormalize
            } else {
                utils::convergent_denormalize
            };
            denormalizer(normalized, self.plain_min, self.plain_max, self.factor)
        }
    }

    fn normalize(&self, plain: f64) -> f64 {
        if plain == self.plain_zero {
            0.0
        } else if plain == self.plain_one {
            1.0
        } else {
            let normalizer = if self.diverge {
                utils::divergent_normalize
            } else {
                utils::convergent_normalize
            };
            normalizer(plain, self.plain_min, self.plain_max, self.factor)
        }
    }

    fn format(&self, normalized: f64) -> String {
        format!("{:.3}", self.denormalize(normalized))
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Some(vs) = string.split(' ').nth(0) {
            if let Ok(v) = vs.parse() {
                Some(self.normalize(v))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct LinearParameter {
    min: f64,
    max: f64,
}

impl Normalizable<f64> for LinearParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        utils::linear_denormalize(normalized, self.min, self.max)
    }

    fn normalize(&self, plain: f64) -> f64 {
        utils::linear_normalize(plain, self.min, self.max)
    }

    fn format(&self, normalized: f64) -> String {
        format!("{:.2}", self.denormalize(normalized))
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Some(vs) = string.split(' ').nth(0) {
            if let Ok(v) = vs.parse() {
                Some(self.normalize(v))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct IntegerParameter {
    min: i32,
    max: i32,
}

impl Normalizable<f64> for IntegerParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        utils::linear_denormalize(normalized, self.min as f64, self.max as f64) as i64 as f64
    }

    fn normalize(&self, plain: f64) -> f64 {
        utils::linear_normalize(plain, self.min as f64, self.max as f64)
    }

    fn format(&self, normalized: f64) -> String {
        format!("{:.2}", self.denormalize(normalized))
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Some(vs) = string.split(' ').nth(0) {
            if let Ok(v) = vs.parse() {
                Some(self.normalize(v))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct ListParameter {
    elements: &'static [&'static str],
}

impl Normalizable<f64> for ListParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        normalized * (self.elements.len() - 1) as f64
    }

    fn normalize(&self, plain: f64) -> f64 {
        plain / (self.elements.len() - 1) as f64
    }

    fn format(&self, normalized: f64) -> String {
        if let Some(s) = self.elements.get(self.denormalize(normalized) as usize) {
            format!("{}", s)
        } else {
            "".to_string()
        }
    }

    fn parse(&self, string: &str) -> Option<f64> {
        if let Ok(v) = self.elements.binary_search(&string) {
            Some(self.normalize(v as f64))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub union ParameterInfo {
    pub non_linear: NonLinearParameter,
    pub linear: LinearParameter,
    pub list: ListParameter,
    pub int: IntegerParameter,
}

#[derive(Clone)]
pub struct SoyBoyParameter {
    pub r#type: ParameterType,
    pub parameter: ParameterInfo,
    pub title: String,
    pub short_title: String,
    pub unit_name: String,
    pub step_count: i32,
    pub default_value: f64,
}

impl Normalizable<f64> for SoyBoyParameter {
    fn denormalize(&self, normalized: f64) -> f64 {
        match self.r#type {
            ParameterType::NonLinear => unsafe {
                self.parameter.non_linear.denormalize(normalized)
            },
            ParameterType::Linear => unsafe { self.parameter.linear.denormalize(normalized) },
            ParameterType::List => unsafe { self.parameter.list.denormalize(normalized) },
            ParameterType::Integer => unsafe { self.parameter.int.denormalize(normalized) },
        }
    }

    fn normalize(&self, plain: f64) -> f64 {
        match self.r#type {
            ParameterType::NonLinear => unsafe { self.parameter.non_linear.normalize(plain) },
            ParameterType::Linear => unsafe { self.parameter.linear.normalize(plain) },
            ParameterType::List => unsafe { self.parameter.list.normalize(plain) },
            ParameterType::Integer => unsafe { self.parameter.int.normalize(plain) },
        }
    }

    fn format(&self, normalized: f64) -> String {
        let s = match self.r#type {
            ParameterType::NonLinear => unsafe { self.parameter.non_linear.format(normalized) },
            ParameterType::Linear => unsafe { self.parameter.linear.format(normalized) },
            ParameterType::List => unsafe { self.parameter.list.format(normalized) },
            ParameterType::Integer => unsafe { self.parameter.int.format(normalized) },
        };
        format!("{} {}", s, self.unit_name)
    }

    fn parse(&self, string: &str) -> Option<f64> {
        match self.r#type {
            ParameterType::NonLinear => unsafe { self.parameter.non_linear.parse(string) },
            ParameterType::Linear => unsafe { self.parameter.linear.parse(string) },
            ParameterType::List => unsafe { self.parameter.list.parse(string) },
            ParameterType::Integer => unsafe { self.parameter.int.parse(string) },
        }
    }
}

fn make_global_parameters(params: &mut HashMap<Parameter, SoyBoyParameter>) {
    static GLOBAL_MASTER_VOLUME: NonLinearParameter = NonLinearParameter {
        plain_zero: -f64::INFINITY,
        plain_min: -110.0,
        plain_max: 6.0,
        plain_one: 6.0,
        factor: 10.0,
        diverge: false,
    };
    params.insert(
        Parameter::MasterVolume,
        SoyBoyParameter {
            r#type: ParameterType::NonLinear,
            parameter: ParameterInfo {
                non_linear: GLOBAL_MASTER_VOLUME,
            },
            title: "Master Volume".to_string(),
            short_title: "Volume".to_string(),
            unit_name: "dB".to_string(),
            step_count: 0,
            default_value: -4.0,
        },
    );

    static GLOBAL_PITCH: IntegerParameter = IntegerParameter {
        min: -200,
        max: 200,
    };
    params.insert(
        Parameter::Pitch,
        SoyBoyParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo { int: GLOBAL_PITCH },
            title: "Pitch".to_string(),
            short_title: "Pitch".to_string(),
            unit_name: "cent".to_string(),
            step_count: GLOBAL_PITCH.max.abs() + GLOBAL_PITCH.min.abs(),
            default_value: 0.0,
        },
    );

    static SELECTED_OSCILLATOR_LIST: [&str; 3] = ["Square", "Noise", "Wavetable"];
    static SELECTED_OSC: ListParameter = ListParameter {
        elements: &SELECTED_OSCILLATOR_LIST,
    };
    params.insert(
        Parameter::OscillatorType,
        SoyBoyParameter {
            r#type: ParameterType::List,
            parameter: ParameterInfo { list: SELECTED_OSC },
            title: "Osc type".to_string(),
            short_title: "Osc type".to_string(),
            unit_name: "".to_string(),
            step_count: (SELECTED_OSC.denormalize(1.0)) as i32,
            default_value: 0.0,
        },
    );
}

pub fn make_square_oscillator_parameters(params: &mut HashMap<Parameter, SoyBoyParameter>) {
    static SQUARE_OSCILLATOR_DUTY_LIST: [&str; 3] = ["12.5%", "25%", "50%"];
    static OSC_SQ_DUTY: ListParameter = ListParameter {
        elements: &SQUARE_OSCILLATOR_DUTY_LIST,
    };
    params.insert(
        Parameter::OscSqDuty,
        SoyBoyParameter {
            r#type: ParameterType::List,
            parameter: ParameterInfo { list: OSC_SQ_DUTY },
            title: "OscSq: Duty".to_string(),
            short_title: "Duty".to_string(),
            unit_name: "".to_string(),
            step_count: (OSC_SQ_DUTY.denormalize(1.0)) as i32,
            default_value: 2.0,
        },
    );
    static SQUARE_OSCILLATOR_SWEEP_TYPE_LIST: [&str; 4] = ["None", "Up", "Down", "Tri"];
    static OSC_SQ_SWEEP_TYPE: ListParameter = ListParameter {
        elements: &SQUARE_OSCILLATOR_SWEEP_TYPE_LIST,
    };
    params.insert(
        Parameter::OscSqSweepType,
        SoyBoyParameter {
            r#type: ParameterType::List,
            parameter: ParameterInfo {
                list: OSC_SQ_SWEEP_TYPE,
            },
            title: "OscSq: Sweep Type".to_string(),
            short_title: "Sweep Type".to_string(),
            unit_name: "".to_string(),
            step_count: (OSC_SQ_SWEEP_TYPE.denormalize(1.0)) as i32,
            default_value: 0.0,
        },
    );
    static OSC_SQ_SWEEP_AMOUNT: IntegerParameter = IntegerParameter { min: 0, max: 8 };
    params.insert(
        Parameter::OscSqSweepAmount,
        SoyBoyParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo {
                int: OSC_SQ_SWEEP_AMOUNT,
            },
            title: "OscSq: Sweep Amount".to_string(),
            short_title: "Sweep Amount".to_string(),
            unit_name: "".to_string(),
            step_count: OSC_SQ_SWEEP_AMOUNT.max - OSC_SQ_SWEEP_AMOUNT.min,
            default_value: 0.0,
        },
    );
    static OSC_SQ_SWEEP_PERIOD: IntegerParameter = IntegerParameter { min: 0, max: 8 };
    params.insert(
        Parameter::OscSqSweepPeriod,
        SoyBoyParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo {
                int: OSC_SQ_SWEEP_PERIOD,
            },
            title: "OscSq: Sweep period".to_string(),
            short_title: "Sweep period".to_string(),
            unit_name: "".to_string(),
            step_count: OSC_SQ_SWEEP_PERIOD.max - OSC_SQ_SWEEP_PERIOD.min - 1,
            default_value: 0.0,
        },
    );
}

pub fn make_noise_oscillator_parameters(params: &mut HashMap<Parameter, SoyBoyParameter>) {
    static OSC_NS_INTERVAL: NonLinearParameter = NonLinearParameter {
        plain_zero: 0.001,
        plain_min: 0.002,
        plain_max: 1.0,
        plain_one: 1.0,
        factor: 2.0,
        diverge: true,
    };
    params.insert(
        Parameter::OscNsInterval,
        SoyBoyParameter {
            r#type: ParameterType::NonLinear,
            parameter: ParameterInfo {
                non_linear: OSC_NS_INTERVAL,
            },
            title: "OscNs: Noise interval".to_string(),
            short_title: "Noise int".to_string(),
            unit_name: "ms".to_string(),
            step_count: 0,
            default_value: 0.05,
        },
    );
}

pub fn make_wavetable_oscillator_parameters(params: &mut HashMap<Parameter, SoyBoyParameter>) {
    static OSC_WT_TABLE_INDEX: IntegerParameter = IntegerParameter { min: 0, max: 31 };
    params.insert(
        Parameter::OscWtTableIndex,
        SoyBoyParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo {
                int: OSC_WT_TABLE_INDEX,
            },
            title: "OscWt: Table index".to_string(),
            short_title: "Table index".to_string(),
            unit_name: "".to_string(),
            step_count: OSC_WT_TABLE_INDEX.max - OSC_WT_TABLE_INDEX.min - 1,
            default_value: 0.0,
        },
    );

    static OSC_WT_TABLE_VALUE: IntegerParameter = IntegerParameter { min: -15, max: 16 };
    params.insert(
        Parameter::OscWtTableValue,
        SoyBoyParameter {
            r#type: ParameterType::Integer,
            parameter: ParameterInfo {
                int: OSC_WT_TABLE_VALUE,
            },
            title: "OscWt: Table value".to_string(),
            short_title: "Table value".to_string(),
            unit_name: "".to_string(),
            step_count: OSC_WT_TABLE_VALUE.max - OSC_WT_TABLE_VALUE.min - 1,
            default_value: 0.0,
        },
    );
}

pub fn make_envelope_generator_parameters(params: &mut HashMap<Parameter, SoyBoyParameter>) {
    static EG_TIME: NonLinearParameter = NonLinearParameter {
        plain_zero: 0.00,
        plain_min: 0.01,
        plain_max: 2.0,
        plain_one: 2.0,
        factor: 1.4,
        diverge: true,
    };
    params.insert(
        Parameter::EgAttack,
        SoyBoyParameter {
            r#type: ParameterType::NonLinear,
            parameter: ParameterInfo {
                non_linear: EG_TIME,
            },
            title: "Eg: Attack".to_string(),
            short_title: "Attack".to_string(),
            unit_name: "s".to_string(),
            step_count: 0,
            default_value: 0.08,
        },
    );
    params.insert(
        Parameter::EgDecay,
        SoyBoyParameter {
            r#type: ParameterType::NonLinear,
            parameter: ParameterInfo {
                non_linear: EG_TIME,
            },
            title: "Eg: Decay".to_string(),
            short_title: "Decay".to_string(),
            unit_name: "s".to_string(),
            step_count: 0,
            default_value: 0.1,
        },
    );
    params.insert(
        Parameter::EgRelease,
        SoyBoyParameter {
            r#type: ParameterType::NonLinear,
            parameter: ParameterInfo {
                non_linear: EG_TIME,
            },
            title: "Eg: Release".to_string(),
            short_title: "Release".to_string(),
            unit_name: "s".to_string(),
            step_count: 0,
            default_value: 0.1,
        },
    );
    static EG_SUSTAIN: LinearParameter = LinearParameter { min: 0.0, max: 1.0 };
    params.insert(
        Parameter::EgSustain,
        SoyBoyParameter {
            r#type: ParameterType::Linear,
            parameter: ParameterInfo { linear: EG_SUSTAIN },
            title: "Eg: Sustain".to_string(),
            short_title: "Sustain".to_string(),
            unit_name: "".to_string(),
            step_count: 0,
            default_value: 0.3,
        },
    );
}

pub fn make_parameter_info() -> HashMap<Parameter, SoyBoyParameter> {
    let mut params = HashMap::new();

    make_global_parameters(&mut params);

    make_square_oscillator_parameters(&mut params);
    make_noise_oscillator_parameters(&mut params);
    make_wavetable_oscillator_parameters(&mut params);

    make_envelope_generator_parameters(&mut params);

    params
}
