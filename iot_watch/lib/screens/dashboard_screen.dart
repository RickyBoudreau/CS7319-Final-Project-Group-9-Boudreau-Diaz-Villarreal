import 'package:flutter/material.dart';
import 'package:iot_watch/domain/sensor_repository.dart';
import 'package:iot_watch/screens/health_screen.dart';
import 'package:iot_watch/screens/water_screen.dart';
import 'package:iot_watch/screens/weather_screen.dart';
import 'package:iot_watch/screens/message_screen.dart';

class WatchWidgetData {
  final String id;
  final IconData icon;
  final Color backgroundColor;
  final Color iconColor;
  final Widget targetScreen;

  WatchWidgetData({
    required this.id,
    required this.icon,
    required this.backgroundColor,
    required this.iconColor,
    required this.targetScreen,
  });
}

class WatchDashboard extends StatelessWidget {
  final SensorRepository repository;

  const WatchDashboard({super.key, required this.repository});

  @override
  Widget build(BuildContext context) {
    final List<WatchWidgetData> widgets = [
      WatchWidgetData(
        id: 'messages',
        icon: Icons.chat_bubble_outline,
        backgroundColor: const Color(0xFF1EAD36),
        iconColor: Colors.white,
        targetScreen: MessagesAppScreen(repository: repository),
      ),
      WatchWidgetData(
        id: 'weather',
        icon: Icons.wb_sunny_outlined,
        backgroundColor: const Color(0xFF63B4FF),
        iconColor: Colors.yellowAccent,
        targetScreen: WeatherAppScreen(repository: repository),
      ),
      WatchWidgetData(
        id: 'health',
        icon: Icons.favorite_border,
        backgroundColor: const Color(0xFFFFF5F5),
        iconColor: Colors.red,
        targetScreen: HealthAppScreen(repository: repository),
      ),
      WatchWidgetData(
        id: 'water',
        icon: Icons.water_drop_outlined,
        backgroundColor: const Color(0xFF0044CC),
        iconColor: Colors.white,
        targetScreen: WaterAppScreen(repository: repository),
      ),
    ];

    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: GridView.builder(
        physics: const NeverScrollableScrollPhysics(),
        gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: 2,
          crossAxisSpacing: 8.0,
          mainAxisSpacing: 8.0,
        ),
        itemCount: widgets.length,
        itemBuilder: (context, index) {
          final data = widgets[index];
          return GestureDetector(
            onTap: () async {
              // 1. Tell Rust the app is opening
              repository.openApp(data.id);
              
              // 2. Wait while the user is inside the app
              await Navigator.of(context).push(
                MaterialPageRoute(builder: (context) => data.targetScreen),
              );
              
              // 3. Tell Rust the app closed when they pop back to the dashboard
              repository.closeApp(data.id);
            },
            child: Hero(
              tag: data.id,
              child: Container(
                decoration: BoxDecoration(
                  color: data.backgroundColor,
                  borderRadius: BorderRadius.circular(16.0),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withValues(alpha: 0.2),
                      blurRadius: 4,
                      offset: const Offset(0, 2),
                    ),
                  ],
                ),
                child: Center(
                  child: Icon(data.icon, color: data.iconColor, size: 40.0),
                ),
              ),
            ),
          );
        },
      ),
    );
  }
}
