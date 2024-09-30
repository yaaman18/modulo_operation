// Base58エンコード用の文字セット
const BASE58_CHARS: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Base58の文字列を整数に変換する関数
fn base58_to_int(base58: &str) -> u128 {
    let mut result: u128 = 0;
    for c in base58.chars() {
        if let Some(index) = BASE58_CHARS.find(c) {
            result = result * 58 + index as u128;
        } else {
            panic!("Invalid Base58 character: {}", c);
        }
    }
    result
}

// 整数をBase58の文字列に変換する関数
fn int_to_base58(mut n: u128) -> String {
    if n == 0 {
        return "1".to_string();
    }
    let mut result = String::new();
    while n > 0 {
        let rem = (n % 58) as usize;
        result.push(BASE58_CHARS.chars().nth(rem).unwrap());
        n /= 58;
    }
    result.chars().rev().collect()
}

// 整数を3進数のベクタに変換する関数（60桁）
fn int_to_trinary(mut n: u128) -> Vec<u8> {
    let mut trinary = vec![0; 60];
    for i in (0..60).rev() {
        trinary[i] = (n % 3) as u8;
        n /= 3;
    }
    trinary
}

// 中国剰余定理による再構築
fn chinese_remainder_theorem(r: &[u128], m: &[u128]) -> u128 {
    let m_product: u128 = m.iter().product();
    let mut result = 0u128;
    for (&r_i, &m_i) in r.iter().zip(m.iter()) {
        let mi = m_product / m_i;
        let inv = mod_inverse(mi % m_i, m_i);
        result = (result + r_i * mi % m_product * inv % m_product) % m_product;
    }
    result
}

// モジュロ逆数を計算する関数 (拡張ユークリッドの互除法)
fn mod_inverse(a: u128, m: u128) -> u128 {
    let (mut a, mut m) = (a as i128, m as i128);
    let m0 = m;
    let (mut x0, mut x1) = (0i128, 1i128);

    if m == 1 {
        return 0;
    }

    while a > 1 {
        let q = a / m;
        let t = m;
        m = a % m;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += m0;
    }

    x1 as u128
}

// モジュラ演算を行う関数
fn mod_operation(n: u128, mods: &[u128]) -> Vec<u128> {
    mods.iter().map(|&p| n % p).collect()
}

// 3×3のマトリクスを点（1/0）で表現
fn matrix_representation(value: u128) -> [u8; 9] {
    let mut matrix = [0; 9]; // 0で初期化
    let first_pos = (value % 9) as usize; // 余りを位置に変換
    let second_pos = ((value / 9) % 9) as usize; // 商を別の位置に変換

    matrix[first_pos] = 1;  // 余りの位置に点を打つ
    matrix[second_pos] = 1; // 商の位置に点を打つ

    matrix
}

// マトリクスを1列に並べて表示
fn print_matrix(matrix: [u8; 9]) {
    for &cell in matrix.iter() {
        print!("{}", cell);  // 各セルの値を1行で表示
    }
    println!(); // 改行
}

fn main() {
    // Base58の16文字の文字列を定義
    let provided_code = "3KMUV7snH6wU48zt"; // 例のBase58文字列

    // Base58を整数に変換
    let base58_integer = base58_to_int(provided_code);
    println!("Base58から変換された整数: {}", base58_integer);

    // 整数を3進法の60桁のベクタに変換
    let trinary_data = int_to_trinary(base58_integer);

    // 3進法の数列を整数に変換
    let mut n: u128 = 0;
    for &val in &trinary_data {
        n = n * 3 + val as u128;
    }

    // モジュラ演算に使う大きな素数
    let p1: u128 = 1000000007;
    let p2: u128 = 1000000009;
    let p3: u128 = 1000000021;

    // モジュラ演算の実行
    let primes = [p1, p2, p3];
    let remainders = mod_operation(n, &primes);

    // 結果を3×3のマトリクスで表現
    for (i, &r) in remainders.iter().enumerate() {
        println!("r{} ({}):", i + 1, r);
        let matrix = matrix_representation(r);
        print_matrix(matrix);
    }

    // 中国剰余定理で元の整数を再構築
    let reconstructed_n = chinese_remainder_theorem(&remainders, &primes);
    println!("再構築された整数: {}", reconstructed_n);

    // 再構築された整数を3進法のベクタに戻す
    let mut restored_trinary = vec![0u8; 60];
    let mut temp_n = reconstructed_n;
    for i in (0..60).rev() {
        restored_trinary[i] = (temp_n % 3) as u8;
        temp_n /= 3;
    }

    // トリナリーを整数に戻す
    let mut restored_base58_integer: u128 = 0;
    for &val in &restored_trinary {
        restored_base58_integer = restored_base58_integer * 3 + val as u128;
    }

    // 再構築された整数をBase58に変換
    let restored_base58 = int_to_base58(restored_base58_integer);
    println!("復元されたBase58文字列: {}", restored_base58);
}

//実用性なし。供養