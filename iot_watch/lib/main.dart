import 'package:flutter/material.dart';

void main() {
  runApp(const IoTWatchApp());
}

class IoTWatchApp extends StatelessWidget {
  const IoTWatchApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'IoT Watch UI',
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF333333),
      ),
      // THE FIX: Constrain the entire app's navigation stack here
      builder: (context, child) {
        return Scaffold(
          // Optional: A darker background for your web browser so the watch "pops"
          backgroundColor: Colors.black,
          body: Center(
            child: ClipRRect(
              // Optional: Add rounded corners to simulate the physical watch casing
              borderRadius: BorderRadius.circular(32.0),
              child: SizedBox(
                width: 200,
                height: 200,
                // 'child' represents whatever screen the Navigator is currently showing
                child: child,
              ),
            ),
          ),
        );
      },
      // Now we just pass the dashboard directly without the wrapper here
      home: const WatchDashboard(),
    );
  }
}

// UPDATED: Added targetScreen to hold the specific app widget
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
  const WatchDashboard({super.key});

  @override
  Widget build(BuildContext context) {
    // UPDATED: Wiring up the specific placeholder screens
    final List<WatchWidgetData> widgets = [
      WatchWidgetData(
        id: 'messages',
        icon: Icons.chat_bubble_outline,
        backgroundColor: const Color(0xFF1EAD36),
        iconColor: Colors.white,
        targetScreen: const MessagesAppScreen(), // Destination
      ),
      WatchWidgetData(
        id: 'weather',
        icon: Icons.wb_sunny_outlined,
        backgroundColor: const Color(0xFF63B4FF),
        iconColor: Colors.yellowAccent,
        targetScreen: const WeatherAppScreen(), // Destination
      ),
      WatchWidgetData(
        id: 'health',
        icon: Icons.favorite_border,
        backgroundColor: const Color(0xFFFFF5F5),
        iconColor: Colors.red,
        targetScreen: const HealthAppScreen(), // Destination
      ),
      WatchWidgetData(
        id: 'water',
        icon: Icons.water_drop_outlined,
        backgroundColor: const Color(0xFF0044CC),
        iconColor: Colors.white,
        targetScreen: const WaterAppScreen(), // Destination
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
          return WidgetButton(data: widgets[index]);
        },
      ),
    );
  }
}

class WidgetButton extends StatelessWidget {
  final WatchWidgetData data;

  const WidgetButton({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () {
        // UPDATED: Push the specific target screen instead of a generic view
        Navigator.of(
          context,
        ).push(MaterialPageRoute(builder: (context) => data.targetScreen));
      },
      child: Hero(
        tag: data.id,
        child: Container(
          decoration: BoxDecoration(
            color: data.backgroundColor,
            borderRadius: BorderRadius.circular(16.0),
          ),
          child: Center(
            child: Icon(data.icon, color: data.iconColor, size: 40.0),
          ),
        ),
      ),
    );
  }
}

// =====================================================================
// PLACEHOLDER SCREENS
// You will replace the internals of these with your wireframe designs
// =====================================================================

class MessagesAppScreen extends StatelessWidget {
  const MessagesAppScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return PlaceholderScreenTemplate(
      title: 'Messages App',
      backgroundColor: const Color(0xFF1EAD36),
      icon: Icons.chat_bubble_outline,
    );
  }
}

class WeatherAppScreen extends StatelessWidget {
  const WeatherAppScreen({super.key});

  @override
  Widget build(BuildContext context) {
    const String currentTemp = '78';
    const String highTemp = '81';
    const String lowTemp = '64';
    const String barometricPressure = '734';

    return Scaffold(
      backgroundColor: const Color(0xFF333333), 
      body: GestureDetector(
        onTap: () => Navigator.of(context).pop(),
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Padding(
                padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                child: Text(
                  'Weather',
                  style: TextStyle(color: Colors.white70, fontSize: 14),
                ),
              ),
              
              Expanded(
                child: Container(
                  width: double.infinity,
                  decoration: BoxDecoration(
                    color: const Color(0xFFF2ECEC), 
                    borderRadius: BorderRadius.circular(20),
                  ),
                  // THE FIX: Reduced vertical padding from 12.0 to 8.0. 
                  // This reclaims 8 total pixels of vertical space, easily fixing a 1px overflow.
                  padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 8.0),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                    children: [
                      Column(
                        children: [
                          Row(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              const SizedBox(
                                width: 44,
                                height: 36,
                                child: Stack(
                                  children: [
                                    Positioned(
                                      top: 0,
                                      left: 0,
                                      child: Icon(Icons.wb_sunny, color: Colors.yellow, size: 26),
                                    ),
                                    Positioned(
                                      bottom: 0,
                                      right: 0,
                                      child: Icon(Icons.cloud_outlined, color: Colors.lightBlue, size: 28),
                                    ),
                                  ],
                                ),
                              ),
                              const SizedBox(width: 8),
                              Text(
                                '$currentTemp°F',
                                style: const TextStyle(
                                  fontSize: 32, 
                                  fontWeight: FontWeight.w500, 
                                  color: Colors.black,
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 2), // Slightly reduced from 4
                          Text(
                            'H: $highTemp° L: $lowTemp°',
                            style: const TextStyle(
                              fontSize: 12, 
                              color: Colors.black87,
                            ),
                          ),
                        ],
                      ),

                      // THE FIX: Reduced divider height from 16 to 12.
                      // Reclaims another 4 pixels to give the bottom text plenty of breathing room.
                      const Divider(
                        color: Colors.grey,
                        thickness: 1,
                        height: 12, 
                      ),

                      Column(
                        children: [
                          const FittedBox(
                            fit: BoxFit.scaleDown,
                            child: Text(
                              'Barometric Pressure',
                              style: TextStyle(fontSize: 10, color: Colors.black87),
                            ),
                          ),
                          const SizedBox(height: 2), // Slightly reduced from 4
                          FittedBox(
                            fit: BoxFit.scaleDown,
                            child: RichText(
                              text: TextSpan(
                                children: [
                                  TextSpan(
                                    text: barometricPressure, 
                                    style: const TextStyle(fontSize: 24, color: Colors.black)
                                  ),
                                  const TextSpan(
                                    text: ' mmHG', 
                                    style: TextStyle(fontSize: 12, color: Colors.black)
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class HealthAppScreen extends StatelessWidget {
  const HealthAppScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // These variables act as placeholders for your future Rust FFI integration.
    // Eventually, you will likely wrap this widget in a StreamBuilder or
    // StateNotifier to listen to the live sensor data.
    const String heartRate = '87';
    const String bloodPressure = '120/80';
    const String currentDate = '03/04/2026';
    const String currentTime = '09:54:05';
    const String dailySteps = '8,987';
    const String distance = '~23,366';

    return Scaffold(
      backgroundColor: const Color(
        0xFF333333,
      ), // Dark background matching the watch face
      body: GestureDetector(
        // Included so you can still tap to go back during testing
        onTap: () => Navigator.of(context).pop(),
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Top "Health" Title
              const Padding(
                padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                child: Text(
                  'Health',
                  style: TextStyle(color: Colors.white70, fontSize: 14),
                ),
              ),

              // Main Application Card
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    color: const Color(
                      0xFFF2ECEC,
                    ), // Light pinkish-off-white background
                    borderRadius: BorderRadius.circular(20),
                  ),
                  padding: const EdgeInsets.all(8.0),
                  child: Column(
                    children: [
                      // Top Row: Vitals Card & Date/Time
                      Expanded(
                        flex: 5,
                        child: Row(
                          children: [
                            // White Inner Card for Heart Rate & BP
                            // White Inner Card for Heart Rate & BP
                            Expanded(
                              flex: 11,
                              child: Container(
                                // REDUCED PADDING: from 8.0 to 4.0 to give the text more breathing room
                                padding: const EdgeInsets.all(4.0),
                                decoration: BoxDecoration(
                                  color: Colors.white,
                                  borderRadius: BorderRadius.circular(16),
                                ),
                                // ADDED FITTEDBOX: This prevents the yellow/black overflow tape
                                child: FittedBox(
                                  fit: BoxFit.scaleDown,
                                  alignment: Alignment.centerLeft,
                                  child: Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    mainAxisAlignment:
                                        MainAxisAlignment.spaceEvenly,
                                    children: [
                                      const Row(
                                        mainAxisAlignment:
                                            MainAxisAlignment.spaceBetween,
                                        children: [
                                          Text(
                                            'Heart Rate',
                                            style: TextStyle(
                                              fontSize: 10,
                                              color: Colors.black87,
                                            ),
                                          ),
                                          SizedBox(
                                            width: 4,
                                          ), // Added slight spacing before the icon
                                          Icon(
                                            Icons.favorite_border,
                                            color: Colors.red,
                                            size: 14,
                                          ),
                                        ],
                                      ),
                                      RichText(
                                        text: const TextSpan(
                                          children: [
                                            TextSpan(
                                              text: heartRate,
                                              style: TextStyle(
                                                fontSize: 22,
                                                fontWeight: FontWeight.w500,
                                                color: Colors.black,
                                              ),
                                            ),
                                            TextSpan(
                                              text: ' BPM',
                                              style: TextStyle(
                                                fontSize: 9,
                                                color: Colors.black54,
                                              ),
                                            ),
                                          ],
                                        ),
                                      ),
                                      const SizedBox(
                                        height: 2,
                                      ), // Tiny spacer instead of relying purely on spaceEvenly
                                      const Text(
                                        'Blood Pressure',
                                        style: TextStyle(
                                          fontSize: 10,
                                          color: Colors.black87,
                                        ),
                                      ),
                                      const Text(
                                        bloodPressure,
                                        style: TextStyle(
                                          fontSize: 18,
                                          color: Colors.black,
                                        ),
                                      ),
                                    ],
                                  ),
                                ),
                              ),
                            ),
                            const SizedBox(width: 8),

                            // Right Side: Date and Time
                            const Expanded(
                              flex: 9,
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Text(
                                    'Today',
                                    style: TextStyle(
                                      fontSize: 18,
                                      fontWeight: FontWeight.bold,
                                      color: Colors.black,
                                    ),
                                  ),
                                  SizedBox(height: 6),
                                  Text(
                                    currentDate,
                                    style: TextStyle(
                                      fontSize: 11,
                                      color: Colors.black87,
                                    ),
                                  ),
                                  SizedBox(height: 2),
                                  Text(
                                    currentTime,
                                    style: TextStyle(
                                      fontSize: 11,
                                      color: Colors.black87,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),

                      const SizedBox(height: 8),

                      // Bottom Row: Steps & Distance
                      Expanded(
                        flex: 3,
                        child: Row(
                          children: [
                            Expanded(
                              child: Padding(
                                padding: const EdgeInsets.only(left: 4.0),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  children: [
                                    const Text(
                                      'Daily Steps',
                                      style: TextStyle(
                                        fontSize: 10,
                                        color: Colors.black87,
                                      ),
                                    ),
                                    RichText(
                                      text: const TextSpan(
                                        children: [
                                          TextSpan(
                                            text: dailySteps,
                                            style: TextStyle(
                                              fontSize: 16,
                                              color: Colors.black,
                                            ),
                                          ),
                                          TextSpan(
                                            text: ' steps',
                                            style: TextStyle(
                                              fontSize: 9,
                                              color: Colors.black54,
                                            ),
                                          ),
                                        ],
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                            Expanded(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.start,
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  const Text(
                                    'Distance',
                                    style: TextStyle(
                                      fontSize: 10,
                                      color: Colors.black87,
                                    ),
                                  ),
                                  RichText(
                                    text: const TextSpan(
                                      children: [
                                        TextSpan(
                                          text: distance,
                                          style: TextStyle(
                                            fontSize: 16,
                                            color: Colors.black,
                                          ),
                                        ),
                                        TextSpan(
                                          text: ' ft',
                                          style: TextStyle(
                                            fontSize: 9,
                                            color: Colors.black54,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class WaterAppScreen extends StatelessWidget {
  const WaterAppScreen({super.key});

  @override
  Widget build(BuildContext context) {
    // Placeholders for your future Rust FFI integration.
    // These will likely become stream yields or state variables later.
    const String waterDetectionStatus = 'No Current Water Detection';
    const String featureStatus = 'Monitoring Device';

    return Scaffold(
      backgroundColor: const Color(0xFF333333), // Matches the watch face background
      body: GestureDetector(
        // Tap anywhere to go back to the dashboard during testing
        onTap: () => Navigator.of(context).pop(),
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Top Title
              const Padding(
                padding: EdgeInsets.only(left: 4.0, bottom: 4.0),
                child: Text(
                  'Water Removal',
                  style: TextStyle(color: Colors.white70, fontSize: 14),
                ),
              ),
              
              // Main Application Card
              Expanded(
                child: Container(
                  width: double.infinity,
                  decoration: BoxDecoration(
                    color: const Color(0xFFF2ECEC), // Light off-white background
                    borderRadius: BorderRadius.circular(20),
                  ),
                  padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 12.0),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      // Top Status Text (Now using the Rust placeholder variable)
                      const FittedBox(
                        fit: BoxFit.scaleDown,
                        child: Text(
                          waterDetectionStatus,
                          style: TextStyle(fontSize: 11, color: Colors.black87),
                        ),
                      ),
                      
                      // Custom Icon Box (Red square with stacked icons)
                      Container(
                        width: 44,
                        height: 44,
                        decoration: BoxDecoration(
                          color: const Color(0xFFC43E3E), // Muted red from the wireframe
                          borderRadius: BorderRadius.circular(10),
                        ),
                        child: const Stack(
                          alignment: Alignment.center,
                          children: [
                            Icon(Icons.water_drop_outlined, color: Colors.white, size: 30),
                            // Slightly shifting the X up to center it in the bulb of the drop
                            Positioned(
                              bottom: 8, 
                              child: Icon(Icons.close, color: Colors.white, size: 14),
                            ),
                          ],
                        ),
                      ),
                      
                      // Bottom Status Column
                      Column(
                        children: [
                          const FittedBox(
                            fit: BoxFit.scaleDown,
                            child: Text(
                              'Water Removal Feature Status:',
                              style: TextStyle(fontSize: 10, color: Colors.black87),
                            ),
                          ),
                          const SizedBox(height: 2),
                          // Feature Status Text (Now using the Rust placeholder variable)
                          FittedBox(
                            fit: BoxFit.scaleDown,
                            child: Text(
                              featureStatus,
                              style: const TextStyle(
                                fontSize: 10, 
                                color: Colors.black87, 
                                fontStyle: FontStyle.italic,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// A reusable template for the placeholders so you can easily navigate back
class PlaceholderScreenTemplate extends StatelessWidget {
  final String title;
  final Color backgroundColor;
  final IconData icon;
  final Color textColor;

  const PlaceholderScreenTemplate({
    super.key,
    required this.title,
    required this.backgroundColor,
    required this.icon,
    this.textColor = Colors.white,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: backgroundColor,
      body: GestureDetector(
        // Tap anywhere to go back to the dashboard during testing
        onTap: () => Navigator.of(context).pop(),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(icon, color: textColor, size: 40),
              const SizedBox(height: 16),
              Text(
                title,
                style: TextStyle(
                  color: textColor,
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  decoration: TextDecoration.none,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                '(Tap to go back)',
                style: TextStyle(
                  color: textColor.withOpacity(0.7),
                  fontSize: 10,
                  decoration: TextDecoration.none,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
