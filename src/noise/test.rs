use crate::noise::deserialize::NoiseParameters;
use crate::noise::normal::NormalNoise;
use crate::random::xoroshiro::XoroshiroRandom;

const SEED: i64 = 0x786b544d6f473757_i64;

#[test]
fn normal_noise_test() {
    let sample = vec![-0.54090486313233870_f64, -0.17220705915382953_f64, -0.30347869326375610_f64, 0.08205665670760691_f64, -0.03608712021268435_f64, 0.26432691230363560_f64, -0.07165190622352266_f64, 0.20407342317023433_f64, -0.39539710674692390_f64, 0.13510248041533615_f64, -0.00423159403225498_f64, -0.30968243530808875_f64, 0.11509779410158977_f64, -0.13566111003951994_f64, 0.10755011706518988_f64, 0.04846289536969445_f64, -0.54372804142024860_f64, -0.64797671968545690_f64, -0.73378800724039080_f64, -0.00399251077599720_f64, 0.22115548857448464_f64, -0.80828401016062060_f64, 0.03902094897981677_f64, 0.02229210309130449_f64, -0.00034412206041905_f64, -0.01577045140602567_f64, -0.06411958713855746_f64, -0.04541067958544320_f64, -0.21086147106842149_f64, 0.18596820608097125_f64, 0.32817260462098435_f64, -0.08208230860096952_f64, -0.15275194818648552_f64, -0.00409141553295630_f64, 0.53915621755629020_f64, 0.22484935204033213_f64, 0.31088155702918885_f64, 0.20491661697726887_f64, 0.15744089792958296_f64, 0.17583599363299013_f64, -0.07006922449346880_f64, 0.08709272696611159_f64, 0.65343792175782680_f64, 0.47444418058372495_f64, -0.55575631190933770_f64, -0.00596569077562968_f64, 0.45341503649335646_f64, 0.08505543513765160_f64, -0.56787060860433980_f64, 0.20689481428953213_f64, -0.41115812027609944_f64, 0.40126434669077570_f64, -0.10306400700790610_f64, 0.41599490964177266_f64, -0.80787771149633810_f64, 0.00681864133910034_f64, -0.41222438729108446_f64, -0.09465366262665742_f64, -0.08835263553973163_f64, 0.39042363827885510_f64, -0.01571631022497851_f64, -0.26816708978453390_f64, 0.02425573863040546_f64, 0.70929999416296630_f64, 0.17513289179070280_f64, 0.06738288441807185_f64, 0.72247710115957600_f64, -0.24899759526871230_f64, 0.02600762113755594_f64, 0.13388401241545650_f64, 0.20839423831938980_f64, -0.26417455342008850_f64, 0.19513086319931366_f64, 0.18712162245296274_f64, 0.20406333257340342_f64, -0.04932534772262556_f64, -0.47268041126274607_f64, 0.16640888836534190_f64, -0.17776119825543250_f64, -0.56689223424593130_f64, -0.48875077992421030_f64, -0.41932425877817453_f64, -0.10441640616090674_f64, -0.28121135834027755_f64, 0.26363901318791840_f64, 0.33634764458199760_f64, 0.48620341206537790_f64, -0.26867717906596450_f64, -0.56208227065313030_f64, 0.07849898132136074_f64, -0.47338122946124140_f64, 0.73825008642664970_f64, 0.35772722435065400_f64, 0.38428624556802465_f64, -0.39995082070020416_f64, -0.45463280101295160_f64, 0.06367825072568230_f64, 0.18877985890016320_f64, 0.17078877256472420_f64, 0.21379154689708996_f64, -0.22261091326560536_f64, 0.13874259155891824_f64, 0.01587280046889737_f64, 0.01720543238103060_f64, -0.52140128846779310_f64, -0.44588846517196834_f64, 0.62296235659489220_f64, 0.28357923142651940_f64, 0.32155636671373106_f64, 0.04503670915193811_f64, 0.19875222466646608_f64, -0.03680567355139712_f64, 0.41564746779150924_f64, 0.66328013918866780_f64, -0.01093928414288879_f64, -0.15248999547433156_f64, -0.54320482222167280_f64, -0.14031596038156285_f64, -0.07454516566540208_f64, 0.61921767803905080_f64, -0.39368634008235287_f64, -0.13177798190480725_f64, -0.17763020395618587_f64, -0.76960845124694140_f64, 0.49255179980626440_f64, -0.34368738658689420_f64, -0.15799406256882664_f64, 0.38639026979634660_f64];
    let noise_data = NoiseParameters {
        first_octave: -10,
        amplitudes: vec![1.5, 0.0, 1.0, 0.0, 0.0, 0.0],
    };

    let mut r = XoroshiroRandom::new(SEED);
    let noise = NormalNoise::new(r.as_mut(), noise_data, true);

    for (i, &s) in sample.iter().enumerate() {
        let (x, y, z) = ((r.next_f64() - 0.5) * 10000_f64, (r.next_f64() - 0.5) * 10000_f64, (r.next_f64() - 0.5) * 10000_f64);
        let v = noise.get_value(x, y, z);

        assert!(f64::abs(v - s) < f64::EPSILON, "[{i}] {s} =?= {v} = noise({x}, {y}, {z})");
    }
}

/* Java MCP-Reborn Sample generation code
    var size = 128;
    var r = new XoroshiroRandomSource(0x786b544d6f473757L);

    var noise = NormalNoise.create(r, BuiltinRegistries.NOISE.getHolder(0).get().value());

    var b = new StringBuilder();
    b.append("\nlet sample = vec![");
    for (int i = 0; i < size; i++) {
        if (i > 0) b.append(", ");
        b.append(String.format("%.17f_f64", noise.getValue((r.nextDouble() - 0.5) * 10000, (r.nextDouble() - 0.5) * 10000, (r.nextDouble() - 0.5) * 10000)));
    }
    b.append("];\n");

    System.out.println(b.toString());
 */