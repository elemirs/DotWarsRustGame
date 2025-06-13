# Dot Wars - Grand Strategy Battle Simulator

Rust ve Bevy ile geliştirilmiş 2D Grand Strategy + Real-time Battle Simulation oyunu.

## 🎮 Oyun Özellikleri

### Grand Strategy Katmanı
- **Province-based World Map**: Bölge tabanlı dünya haritası
- **Resource Management**: Altın, yiyecek, malzeme ve insan gücü yönetimi
- **Building System**: Şehir, çiftlik, maden, kışla gibi yapılar
- **Diplomacy**: Factionlar arası ilişkiler ve anlaşmalar
- **Technology Tree**: Araştırma ve teknoloji geliştirme
- **Turn-based Mechanics**: Sıra tabanlı strateji oynanışı

### Battle Simulation Katmanı
- **Real-time Combat**: Gerçek zamanlı taktiksel savaşlar
- **Unit Types**: Piyade, süvari, okçu, top ve özel birimler
- **Formation System**: Dizi, kolon, kare, kama ve dağınık formasyonlar
- **Morale System**: Birlik morali ve kaçma mekanikleri
- **Terrain Effects**: Arazi etkilerinin savaşa olan etkisi
- **Battle AI**: Akıllı savaş yapay zekası

## 🏗️ Teknik Mimari

### 🎯 **Hibrit CPU + GPU Sistem Yaklaşımı**
DotWars, modern oyun geliştirme prensiplerini takip eden **hibrit bir CPU + GPU sistem** ile tasarlanmıştır:

#### **🔧 CPU İşlemleri (Compute-Heavy)**
- **Game Logic**: Core oyun mekaniği ve kurallar
- **AI Systems**: Yapay zeka hesaplamaları ve karar verme
- **Battle Calculations**: Savaş simülasyonları ve damage hesaplamaları
- **Strategy Processing**: Grand strategy hesaplamaları ve optimizasyonlar
- **World Simulation**: Province ve resource yönetimi
- **Event Handling**: Input işleme ve game state yönetimi

#### **🎨 GPU İşlemleri (Graphics-Heavy)**
- **Rendering Pipeline**: Bevy + wgpu/Vulkan backend
- **UI Rendering**: Modern kullanıcı arayüzü ve efektler
- **Visual Effects**: Animasyonlar, ışıma efektleri, parçacık sistemleri
- **Shader Processing**: Renk manipülasyonu ve görsel filtreler
- **Batch Rendering**: Verimli sprite ve mesh rendering

#### **⚡ Performans Avantajları**
- **Paralel İşleme**: CPU ve GPU eş zamanlı çalışır
- **Optimized Workload**: Her işlemci türü en iyi olduğu işi yapar
- **Scalability**: Hem düşük hem yüksek end sistemlerde verimli
- **Modern Architecture**: 2025 oyun geliştirme standartlarına uygun

### Modüler Crate Yapısı
```
dot_wars_rust/
├── crates/
│   ├── core/              # Temel veri yapıları ve ID'ler
│   ├── world/             # Dünya simülasyonu ve province sistemi
│   ├── battle/            # Savaş simülatörü ve unit yönetimi
│   ├── strategy/          # Grand strategy mekanikleri (TODO)
│   ├── ui/                # Kullanıcı arayüzü (TODO)
│   ├── graphics/          # Rendering ve görsel efektler (TODO)
│   ├── ai/                # Yapay zeka sistemleri (TODO)
│   └── save_system/       # Kaydetme/yükleme sistemi (TODO)
```

### Teknoloji Stack
- **Game Engine**: Bevy 0.14 (GPU-accelerated ECS)
- **Rendering Backend**: wgpu + Vulkan/DirectX/Metal
- **Architecture**: Hibrit CPU + GPU Entity Component System (ECS)
- **Language**: Rust 2021 Edition (memory-safe, high-performance)
- **Serialization**: Serde + RON (human-readable data)
- **Performance Philosophy**: CPU için compute, GPU için graphics

## 🚀 Geliştirme Roadmap

### ✅ Phase 1: Foundation (TAMAMLANDI)
- [x] Proje yapısı kurulumu
- [x] Bevy integration
- [x] Core data structures
- [x] Basic ECS components
- [x] **Hibrit CPU+GPU mimarisi kurulumu**
- [x] **Modern UI sistemi (GPU-accelerated)**

### 🔄 Phase 2: World System (DEVAM EDIYOR)
- [x] Province-based world map
- [x] Building system
- [x] Resource management
- [ ] World generation improvements
- [ ] Province connections/adjacency

### 📋 Phase 3: Battle System (TODO)
- [ ] Real-time combat implementation
- [ ] Unit movement and positioning
- [ ] Combat calculations
- [ ] Battle UI
- [ ] Terrain effects integration

### 📋 Phase 4: Grand Strategy (TODO)
- [ ] Diplomacy system
- [ ] Technology tree
- [ ] Faction AI
- [ ] Economic simulation
- [ ] Victory conditions

### 📋 Phase 5: Polish & Features (TODO)
- [ ] Graphics and animations
- [ ] Sound system
- [ ] Save/Load functionality
- [ ] Campaign mode
- [ ] Multiplayer support

## 🛠️ Kurulum ve Çalıştırma

### Rust Kurulumu
```bash
# Rust'ı kurun (rustup ile)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Veya Ubuntu/Debian için
sudo apt update
sudo apt install rustup
rustup default stable
```

### Gereksinimler
- Rust 1.70+ (rustc --version)
- Cargo package manager

### Projeyi Çalıştırma
```bash
# Projeyi klonlayın
git clone <repository_url>
cd DotWarsRust

# Bağımlılıkları yükleyin ve çalıştırın
cargo run
```

### Geliştirme
```bash
# Tüm testleri çalıştır
cargo test

# Belirli bir crate'i test et
cargo test -p dot_wars_core

# Release build
cargo build --release
```

## 📁 Crate Detayları

### `dot_wars_core`
Temel veri yapıları, ID türleri ve shared components.
- **FactionId, ProvinceId, UnitId**: Unique identifiers
- **Position, Health, Resources**: Core components
- **Traits**: Updatable, Serializable

### `dot_wars_world`
Dünya simülasyonu ve province yönetimi.
- **Province**: Bölge yapısı (sahiplik, buildings, terrain)
- **Building System**: Farklı yapı türleri ve etkileri
- **WorldMap**: Global state management
- **WorldGenerator**: Prosedürel dünya üretimi

### `dot_wars_battle`
Savaş simülasyonu ve unit management.
- **Unit System**: Farklı birim türleri ve özellikleri
- **Formation System**: Taktiksel formasyonlar
- **Combat System**: Hasar hesaplama ve casualty management
- **Battle AI**: Temel savaş yapay zekası

## 🎯 Oynanış Hedefleri

1. **Strategic Depth**: Karmaşık strateji kararları
2. **Tactical Combat**: Beceri gerektiren savaşlar
3. **Emergent Gameplay**: Oyuncu hikayelerinin oluşması
4. **Moddability**: Kolay modifikasyon desteği
5. **Performance**: Büyük haritalar ve çok sayıda unit

## 🤝 Katkıda Bulunma

1. Projeyi fork edin
2. Feature branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Değişikliklerinizi commit edin (`git commit -m 'Add amazing feature'`)
4. Branch'inizi push edin (`git push origin feature/amazing-feature`)
5. Pull Request açın

## 📝 Lisans

Bu proje MIT lisansı altında lisanslanmıştır.

## 🔗 Kaynaklar

- [Bevy Game Engine](https://bevyengine.org/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [ECS Architecture](https://github.com/SanderMertens/ecs-faq)

---

## 🎮 Oyunun Şu Anki Durumu (13 Haziran 2025)

### ✅ **Çalışan Özellikler:**
- **🇹🇷 Türkçe Karakter Desteği**: NotoSans font ile tam Türkçe destek
  - Büyük ve küçük tüm Türkçe karakterler: ÇĞIİÖŞÜ çğıöşü
  - Ana menü ve tüm UI'da Türkçe metinler
- **Ana Menü**: Bevy built-in UI ile modern ana menü
  - "Oyunu Başlat" butonu (Dünya Haritası'na geçiş)
  - "Seçenekler" butonu (gelecekte geliştirilecek)
  - "Çıkış" butonu (oyundan çıkış)
- **Dünya Haritası Ekranı**: Placeholder UI ile temel navigasyon
- **Plugin Sistemi**: Modüler crate yapısı çalışıyor
- **State Management**: GameState sistemi aktif
- **Bevy 0.14**: Modern oyun motoru entegrasyonu

### 🔄 **Buton Etkileşimleri:**
- Butonlar hover ve click efektlerine sahip
- Ana menüden dünya haritasına geçiş
- Dünya haritasından ana menüye dönüş
- Çıkış butonu ile oyunu kapatma

### 🎯 **Test Etmek İçin:**
1. `cargo run --bin dot_wars` ile oyunu başlatın
2. Ana menüde Türkçe karakterleri gözlemleyin
3. "Oyunu Başlat" ile dünya haritasına geçin
4. Türkçe karakter test metinlerini kontrol edin
5. "Ana Menü" butonu ile geri dönün

### 🔤 **Font Sistemi:**
- **NotoSans-Regular.ttf**: Genel metinler için
- **NotoSans-Bold.ttf**: Başlıklar için  
- Fallback sistem: Font yüklenmezse varsayılan font kullanılır
- Asset klasöründe: `assets/fonts/`

---

**Not**: Bu proje aktif geliştirme aşamasındadır. Özellikler ve API değişebilir.

---

## 🧠 **Teknik Geliştirme Notları**

### 📋 **Hibrit CPU + GPU Sistem Yaklaşımı**
**(13 Haziran 2025 - Mimari Karar)**

DotWars projesi, modern oyun geliştirme standartlarını takip eden **hibrit CPU + GPU sistem yaklaşımı** ile geliştirilmektedir:

#### **🎯 Design Philosophy:**
- **"Right Tool for Right Job"**: Her işlemci türü en iyi olduğu işi yapar
- **Paralel Processing**: CPU ve GPU eş zamanlı çalışarak maximum performans
- **Scalable Architecture**: Düşük ve yüksek end sistemlerde optimize edilmiş performans
- **Future-Proof**: 2025+ oyun geliştirme trendlerine uygun

#### **📊 İş Yükü Dağılımı:**

**CPU Responsibilities (Compute-Intensive):**
- Game state management ve logic processing
- AI decision making ve behavior trees
- Battle calculations ve damage computations
- Resource management ve economic simulations
- Pathfinding algorithms ve spatial queries
- Network synchronization (future multiplayer)

**GPU Responsibilities (Graphics-Intensive):**
- Vulkan/DirectX rendering pipeline
- UI rendering ve modern interface effects
- Particle systems ve visual effects
- Shader-based computations
- Batch rendering optimizations
- Post-processing effects

#### **⚡ Performans Stratejisi:**
- **Framerate Target**: 60 FPS minimum (GPU load)
- **Simulation Target**: 30-60 TPS (CPU load)
- **Memory Management**: Rust'ın zero-cost abstractions
- **Cache Efficiency**: ECS'in data-oriented design
- **GPU Memory**: Efficient texture atlasing ve buffer management

#### **🔮 Gelecek Planlar:**
- WebGPU desteği (cross-platform compatibility)
- Compute shaders für parallel calculations
- Multi-threading optimization (Bevy's parallel systems)
- GPU-driven rendering techniques

Bu yaklaşım, modern AAA oyunların kullandığı standart hibrit mimariyi takip eder ve DotWars'ın hem küçük hem büyük ölçekli savaşları sorunsuz işlemesini sağlar.

---
