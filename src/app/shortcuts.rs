use crate::models::*;

impl super::App {
    /// Jump to AR parameter and increase its value (stays on current row)
    pub fn jump_to_ar_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_AR, true);
    }
    /// Jump to AR parameter and decrease its value (stays on current row)
    pub fn jump_to_ar_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_AR, false);
    }

    /// Jump to D1R parameter and increase its value (stays on current row)
    pub fn jump_to_d1r_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_D1R, true);
    }
    /// Jump to D1R parameter and decrease its value (stays on current row)
    pub fn jump_to_d1r_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_D1R, false);
    }

    /// Jump to D2R parameter and increase its value (stays on current row)
    pub fn jump_to_d2r_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_D2R, true);
    }
    /// Jump to D2R parameter and decrease its value (stays on current row)
    pub fn jump_to_d2r_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_D2R, false);
    }

    /// Jump to RR parameter and increase its value (stays on current row)
    pub fn jump_to_rr_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_RR, true);
    }
    /// Jump to RR parameter and decrease its value (stays on current row)
    pub fn jump_to_rr_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_RR, false);
    }

    /// Jump to MUL parameter and increase its value (stays on current row)
    pub fn jump_to_mul_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_MUL, true);
    }
    /// Jump to MUL parameter and decrease its value (stays on current row)
    pub fn jump_to_mul_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_MUL, false);
    }

    /// Jump to SM (Slot Mask) parameter and increase its value (stays on current row)
    pub fn jump_to_sm_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_SM, true);
    }
    /// Jump to SM (Slot Mask) parameter and decrease its value (stays on current row)
    pub fn jump_to_sm_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_SM, false);
    }

    /// Jump to TL (Total Level) parameter and increase its value (stays on current row)
    pub fn jump_to_tl_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_TL, true);
    }
    /// Jump to TL (Total Level) parameter and decrease its value (stays on current row)
    pub fn jump_to_tl_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_TL, false);
    }

    /// Jump to D1L (Decay 1 Level) parameter and increase its value (stays on current row)
    pub fn jump_to_d1l_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_D1L, true);
    }
    /// Jump to D1L (Decay 1 Level) parameter and decrease its value (stays on current row)
    pub fn jump_to_d1l_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_D1L, false);
    }

    /// Jump to DT (Detune 1) parameter and increase its value (stays on current row)
    pub fn jump_to_dt_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_DT, true);
    }
    /// Jump to DT (Detune 1) parameter and decrease its value (stays on current row)
    pub fn jump_to_dt_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_DT, false);
    }

    /// Jump to DT2 (Detune 2) parameter and increase its value (stays on current row)
    pub fn jump_to_dt2_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_DT2, true);
    }
    /// Jump to DT2 (Detune 2) parameter and decrease its value (stays on current row)
    pub fn jump_to_dt2_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_DT2, false);
    }

    /// Jump to KS (Key Scaling) parameter and increase its value (stays on current row)
    pub fn jump_to_ks_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_KS, true);
    }
    /// Jump to KS (Key Scaling) parameter and decrease its value (stays on current row)
    pub fn jump_to_ks_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_KS, false);
    }

    /// Jump to AMS (Amplitude Modulation Sensitivity) parameter and increase its value (stays on current row)
    pub fn jump_to_ams_and_increase(&mut self) {
        self.jump_to_op_param(PARAM_AMS, true);
    }
    /// Jump to AMS (Amplitude Modulation Sensitivity) parameter and decrease its value (stays on current row)
    pub fn jump_to_ams_and_decrease(&mut self) {
        self.jump_to_op_param(PARAM_AMS, false);
    }
}
