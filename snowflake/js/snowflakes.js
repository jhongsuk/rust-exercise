//시작 시 실행
window.onload = function() {
    //canvas 초기화, canvas 크기
    var canvas = document.getElementById("myCanvas");
    var context = canvas.getContext("2d");
    var width = window.innerWidth;
    var height = window.innerHeight;

    canvas.width = width;
    canvas.height = height;

    //눈송이 입자
    var maxParticle = 100;
    var particles = [];
    var angle = 0;

    function init() {
        for (var i=0; i<maxParticle; i++) {
            particles[i] = {
                x: Math.random() * width, //x좌표
                y: Math.random() * height, //y좌표
                radius: Math.random() * 4 + 1, //radius
                density: Math.random() * maxParticle 
            };
        }     
    }

    //눈송이 그리기
    function draw() {
        context.clearRect(0, 0, width, height); //canvas 영역 화면 지우기
        context.fillStyle = "rgba(255, 255, 255, 0.8)"; //canvas 영역 색 설정
        context.beginPath(); //눈송이 경로 그리기
    
        for (var i=0; i<maxParticle; i++) {
            var particle = particles[i];
            context.moveTo(particle.x, particle.y); //지정된 좌표로 옮기기
            context.arc(particle.x, particle.y, particle.radius, 0, 2*Math.PI, true);
        }
        context.fill();
    }
    
    //눈송이 움직이게 하기 위해, angle은 sin/cosin 함수가 적용되어 눈송이의 수직/수평 이동을 생성
    function update() {
        angle += 0.01;

        for (var i=0; i<maxParticle; i++) {
            var particle = particles[i];

            //x와 y좌표 업데이트, cos 함수에 1을 추가하여 음수값을 방지.
            //particle.y : 눈송이가 자체 밀도를 가지고 있으며, 각각 다르게 만들어져서 아래로 이동
            //particle.x : 반경을 추가하여 무작위로 생성
            particle.y += Math.cos(angle + particle.density) + 1 + particle.radius/2;
            particle.x += Math.sin(angle) * 4;

            //눈송이가 canvas 범위 밖으로 나가는 만큼 위쪽에서 생성, canvas 왼쪽과 오른쪽에서도 눈송이 생성
            if(particle.x > width+5 || particle.x < -5 || particle.y > height) {
                if(i%3 > 0) {
                    particles[i] = {
                        x: Math.random() * width,
                        y: -5,
                        radius: particle.radius, 
                        density: particle.density
                    };
                } else {
                    if(Math.sin(angle) > 0) {
                        particles[i] = {
                            x: -5,
                            y: Math.random() * height,
                            radius: particle.radius, 
                            density: particle.density
                        };
                    } else{
                        particles[i] = {
                            x: width + 5,
                            y: Math.random() * height,
                            radius: particle.radius,
                            density: particle.density
                        };
                    }
                }
            } 
        }
    } 

    function run() {
        draw();
        update();
    }

    //animation loop
    init();
    setInterval(run, 30);
}