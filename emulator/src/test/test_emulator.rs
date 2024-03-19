#[cfg(test)]
mod test_emulator {
    use crate::emulator::{core_sys::MEM_SIZE, instr::instr_to_string, reg_file::PC, CoreSys};

    #[test]
    fn test_one_plus_one() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r1, #1
            // mov r2, #1
            // add r0, r1, r2
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r1, #1
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r2, #1
            0b1110_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // add r0, r1, r2
            0b1110_0001, 0b0000_0000, 0b0001_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0010,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(1), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(2), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 2);
    }

    #[test]
    fn test_cond() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #1
            // mov r1, #1
            // subs r0, r1, #1
            // moveq r2, #1
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r0, #1
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r1, #1
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // subs r0, r1, #1
            0b1110_1101, 0b0000_0000, 0b0010_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // moveq r2, #1
            0b0000_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(1), 1);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 0);
        assert_eq!(sys.dump_cpsr(), 0b0100);
        sys = sys.step();
        assert_eq!(sys.get_reg(2), 1);
    }

    #[test]
    fn test_b() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // b #16
            // mov r1, #1
            // mov r2, #1
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // b #16
            0b1110_0111, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0001_0000,
            // mov r1, #1
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r2, #1
            0b1110_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(15), 16);
        sys = sys.step();
        assert_ne!(sys.get_reg(1), 1);
        assert_eq!(sys.get_reg(2), 1);
    }

    #[test]
    fn test_cond_and_b() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #1
            // cmp r0, #1
            // beq #32
            // mov r1, #1
            // hlt
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r0, #1
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // cmp r0, #1
            0b1110_1101, 0b0000_0001, 0b0101_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // beq #32
            0b0000_0111, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0010_0000,
            // mov r1, #1
            0b1110_0101, 0b0000_0001, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
        ]);
        sys = sys.step();
        assert_eq!(sys.get_reg(0), 1);
        sys = sys.step();
        assert_eq!(sys.dump_cpsr(), 0b0100);
        sys = sys.step();
        assert_eq!(sys.get_reg(15), 32);
        assert_ne!(sys.get_reg(1), 1);
    }
    #[test]
    fn test_fib() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #0
            // mov r1, #1
            // mov r2, #0
            // mov r3, #8
            // LOOP: add r2, r0, r1
            // mov r0, r1
            // mov r1, r2
            // subs r3, r3, #1
            // bne =LOOP
            // hlt
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r0, #0
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // mov r1, #1
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r2, #0
            0b1110_0101, 0b0000_0000, 0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // mov r3, #8
            0b1110_0101, 0b0000_0000, 0b0000_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1000,
            // LOOP: add r2, r0, r1
            0b1110_0001, 0b0000_0000, 0b0001_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r0, r1
            0b1110_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // mov r1, r2
            0b1110_0001, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0010,
            // subs r3, r3, #1
            0b1110_1101, 0b0000_0000, 0b0010_0011, 0b0000_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // bne LOOP
            0b0001_0111, 0b0000_0000, 0b0000_0000, 0b0000_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0010_0000,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
        ]);
        while !sys.halted() {
            println!("{}", instr_to_string(sys.get_next_instr()));
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 21);
    }
    #[test]
    fn test_str_and_ldr() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // sp is r13
            // mov r0, #255
            // sub sp, sp, #8
            // str r0, sp
            // ldr r1, sp
            // hlt
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r0, #255
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b1111_1111,
            // sub sp, sp, #8
            0b1110_0101, 0b0000_0000, 0b0010_1101, 0b0000_1101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1000,
            // str r0, sp
            0b1110_0110, 0b0000_0000, 0b0001_0000, 0b1101_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // ldr r1, sp
            0b1110_0110, 0b0000_0000, 0b0000_0001, 0b1101_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
        ]);
        while !sys.halted() {
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(1), 255);
    }
    #[test]
    fn test_push_pop() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // mov r0, #255
            // push r0
            // pop r1
            // hlt
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r0, #255
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b1111_1111,
            // push r0
            0b1110_0010, 0b0000_0000, 0b0011_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // pop r1
            0b1110_0010, 0b0000_0000, 0b0010_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // push r0
            0b1110_0010, 0b0000_0000, 0b0011_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // push r0
            0b1110_0010, 0b0000_0000, 0b0011_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // pop r3
            0b1110_0010, 0b0000_0000, 0b0010_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // pop r2
            0b1110_0010, 0b0000_0000, 0b0010_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
        ]);
        while !sys.halted() {
            println!("{}", instr_to_string(sys.get_next_instr()));
            sys = sys.step();
            sys.print();
        }
        assert_eq!(sys.get_reg(0), 255);
        assert_eq!(sys.get_reg(1), 255);
        assert_eq!(sys.get_reg(2), 255);
        assert_eq!(sys.get_reg(3), 255);
        assert_eq!(sys.get_reg(13), MEM_SIZE as u64);
    }
    #[test]
    fn test_recursion() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // f(n) = n + f(n - 1)
            // r0 to r3 are caller saved
            // r4 to r9 are callee saved
            // =f is #40 =end is #120
            // mov r1, #16
            // mov r0, #0
            // nop
            // bl =f
            // hlt
            // f: push lr
            // subs r1, r1, #1
            // beq =end
            // push r1
            // bl =f
            // pop r1
            // add r0, r0, r1
            // pop lr
            // b lr
            // end: mov r0, #0
            // pop lr
            // b lr
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mov r1, #16
            0b1110_0101, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0001_0000,
            // mov r0, #0
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // nop
            0b1110_0100, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // bl =f
            0b1110_0111, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0010_1000,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // f: push lr
            0b1110_0010, 0b0000_0000, 0b0011_1110, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // subs r1, r1, #1
            0b1110_1101, 0b0000_0000, 0b0010_0001, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // beq =end
            0b0000_0111, 0b0000_0000, 0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0111_0000,
            // push r1
            0b1110_0010, 0b0000_0000, 0b0011_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // bl =f
            0b1110_0111, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0010_1000,
            // pop r1
            0b1110_0010, 0b0000_0000, 0b0010_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // add r0, r0, r1
            0b1110_0001, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // pop lr
            0b1110_0010, 0b0000_0000, 0b0010_1110, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // b lr
            0b1110_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1110,
            // end: mov r0, #0
            0b1110_0101, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // pop lr
            0b1110_0010, 0b0000_0000, 0b0010_1110, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // b lr
            0b1110_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1110,
        ]);
        while !sys.halted() {
            println!("{}", instr_to_string(sys.get_next_instr()));
            sys = sys.step();
        }
        assert_eq!(sys.get_reg(0), 120);
    }
    #[test]
    fn test_int() {
        let mut sys = CoreSys::new();
        sys = sys.load_mem(vec![
            // add r0, r0, #1
            // add r0, r0, #1
            // hlt
            //COND|fIop__code_______________|red|____rea|_reb|____rec_____________________________________________
            // mvi r0
            0b1110_0001, 0b0000_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
            // add r0, r0, #1
            0b1110_0101, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // add r0, r0, #1
            0b1110_0101, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0001,
            // hlt
            0b1110_0000, 0b0000_0000, 0b0001_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000,
        ]);
        sys = sys.set_int_table(vec![0, 0]);
        let mut first_int = false;
        while !sys.halted() {
            sys = sys.step();
            sys.print();
            if !first_int && sys.get_reg(PC as u64) == 8 {
                sys = sys.interrupt(1, 2);
                first_int = true;
            }
        }
        assert_eq!(sys.get_reg(0), 4);
    }
}