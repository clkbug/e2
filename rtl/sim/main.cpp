#include <cassert>
#include <cstdio>
#include <memory>

#include "Ve2_core_e2_top.h"
#include "verilated.h"
#include "verilated_fst_c.h"

using namespace std;

class UartRx
{
public:
  uint64_t recieving_data;
  bool recieving;
  uint8_t recieving_bit_counter;

  UartRx()
    : recieving_data(0)
    , recieving(false)
    , recieving_bit_counter(0)
  {
  }
  ~UartRx() = default;

  void tick(uint64_t tick, uint8_t rx)
  {
    const uint64_t DELAY_FRAMES = 234 - 1; // same as in e2_top.veryl
    if (tick % (2 * (DELAY_FRAMES + 1)) != 0) {
      return;
    }
    assert(rx == 0 || rx == 1);
    if (!recieving) {
      if (rx == 0) {
        recieving = true;
        recieving_data = 0;
        recieving_bit_counter = 0;
      }
    } else {
      //   recieving_data = (recieving_data << 1) | rx;
      recieving_data = recieving_data | (rx << recieving_bit_counter);
      recieving_bit_counter++;
      if (recieving_bit_counter == 9) {
        assert(rx == 1);
        recieving_data = recieving_data & 0xFF;
        recieving = false;
        recieving_bit_counter = 0;
        printf("%c", this->recieving_data);
        recieving_data = 0;
      }
    }
  }
};

int
main(int argc, char** argv)
{
  unique_ptr<VerilatedContext> contextp(new VerilatedContext);
  contextp->commandArgs(argc, argv);
  unique_ptr<Ve2_core_e2_top> top(new Ve2_core_e2_top{ contextp.get() });

  contextp->timeunit(-9);
  contextp->timeprecision(-9);
  contextp->traceEverOn(true);

  Verilated::traceEverOn(true);
  unique_ptr<VerilatedFstC> tfp(new VerilatedFstC(nullptr));
  top->trace(tfp.get(), 99);
  tfp->open("e2_core.fst");

  top->clk = 0;
  top->rst = 0;
  top->btn1 = 0;
  top->btn2 = 0;

  uint64_t tick = 0;
  bool rst = true;
  UartRx uart_rx;

  while (!contextp->gotFinish()) {
    if (tick > 100) {
      top->btn1 = 1;
      top->btn2 = 1;
      top->rst = 1;
      rst = false;
    }

    top->clk = !top->clk;
    top->eval();
    contextp->timeInc(5);
    tfp->dump(contextp->time());

    tick++;

    if (rst) {
      printf("%d\tRST\n", tick);
      continue;
    }
    if (tick % 2 == 0) {
      //   printf("%d\tRX_TICK\t%d\n", tick, top->uart_rx);
      uart_rx.tick(tick, top->uart_tx);
    }

    if (tick == (1 << 20)) {
      contextp->gotFinish(true);
      break;
    }
  }

  top->final();
  tfp->close();
  return 0;
}