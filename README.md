基于直升机着舰飞行仿真训练研发了六自由度Stewart运动平台。在TwinCAT3实时运行环境实现L. D. Reid等开发的经典、优化和自适应运动感知算法，并在运动平台上进行测试。首先对常规的转弯、加减速和跃升机动飞行进行测试，验证运动感知算法的正确性；然后针对直升机着舰飞行进行测试，优选算法参数，增强运动感觉对飞行员着舰模拟飞行训练的逼真度。六自由度运动平台测试的价值在于：(1) 验证运动平台对提高直升机着舰飞行仿真训练效果的可行性和重要性，(2) 探索选择运动感知算法参数的基本原则和方法，(3) 建议着舰训练应采用的运动感知算法。

mdata cmd code IPEndpoint
mdata run nCase IPEndpoint Ts(ms) verbose

mdata cmd code IPEndpoint
mdata run case IPEndpoint Ts(ms) verbose

cmd code:
      0: NONE   不执行操作
      1: Run    运行状态, 接受输入并驱动平台运行
      2: Test   运行内部测试
      3: Down   平台归低位并停止
      4: Reserve
      5: Home   平台寻位
      6: Center 平台中立（Home 零位置）
      7: Reserve
      8: Best Stop  先平衡而后归位
      9: Stop   停止驱动, 紧急停止!
     10: Reset  故障复位

 case 0: M1 Turn Entry
      1: M2 Throttle Pulse
      2: M3 Pull/PushOver
      3: M4 helicopter shipborne Landing

cargo run --bin mdata run 0 169.254.1.220   // start and run case 0
cargo run --bin mdata cmd 3                 // put down (landing)