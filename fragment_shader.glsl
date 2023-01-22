#version 420 core
  
out vec4 vertexColor; // specify a color output to the fragment shader
in vec4 gl_FragCoord;
uniform uint windowSizeX;
uniform float object_count;
uniform float light_count;
uniform uint windowSizeY;
uniform float iTime;
uniform vec3 cPos;
layout (binding = 0) uniform positions {
    float position[1024];
};

layout (binding = 0) uniform orientations {
    float orientation[4096];
};
layout (binding = 0) uniform lights {
    float light[1024];
};
layout (binding = 0) uniform dims {
    float dimension[1024];
};

float sphereSDF(vec3 p, float r){
    return length(p) - r;
}

float cubeSDF(vec3 p, vec3 b){
    vec3 q = abs(p) - b;
    float r = 0.001;
    float volume = b.x*b.y*b.z;
    float d = length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0)-r - (0.03*volume);
    d*=0.5;
    return d;
}
vec3 normalize(vec3 a){
    return a/ length(a);
}
float clamp(float x, float a,float b){
    return min(b,max(a,x));
}
float clampn(float x){
    return min(1.,max(0.,x));
}

mat3 getMat(int i){
    i*=9;
    return mat3(orientation[i],orientation[i+1], orientation[i+2],orientation[i+3],orientation[i+4],orientation[i+5], orientation[i+6], orientation[i+7], orientation[i+8]);
}
vec3 getVec(int i){
    i*=3;
    return vec3(position[i],position[i+1],position[i+2]);
}
float map(vec3 p){
    float clos = 100.;
    
    for(float i =0; i< object_count; i+=1.){
        int id = int(i);
        vec3 sP = getVec(id); 
        sP = p - sP;
        mat3 oInvOri = getMat(id); 
        sP = oInvOri*sP;
        clos = min(clos, cubeSDF(sP,vec3(dimension[id*3]/2.,dimension[id*3+1]/2.,dimension[id*3+2]/2.)) );
    }
    return clos;
}
float rayCast(vec3 ro, vec3 rd){
    float t = 0.01;

    for(int i =0; i< 150; i++){
        vec3 p = ro + rd*t;

        float m= map(p);
        
        if(abs(m) < 0.01){
            return t;
        }
        if(t > 200.)break;
        t+= m;

    }

    return -1.;
}
float shadowCast(vec3 ro, vec3 rd,float maxd){
    float t = 0.01;
    float mind = 10000000000.;
    for(int i =0; i< 150; i++){
        vec3 p = ro + rd*t;
        float m= map(p);
        mind = min(mind, m/t*8.); 
        if(t >= maxd){
            break;
        }
        t+= m/2.;
    }
    return mind;
}
vec3 calcNorm(vec3 p){
    float delt = 0.0001;
    
    vec3 gr = vec3(map(p+vec3(delt,0.,0.))-map(p-vec3(delt,0.,0.)),
                   map(p+vec3(0.,delt,0.))-map(p-vec3(0.,delt,0.)),
                   map(p+vec3(0.,0.,delt))-map(p-vec3(0.,0.,delt))
    );
    return normalize(gr);
}
void get_light_attributes(int id, out vec3 light_pos, out vec3 light_color, out float light_brightness){
    id*= 7;

    vec3 readlight_pos    = vec3(light[id+0],light[id+1],light[id+2]);
    vec3 readlight_color  = vec3(light[id+3],light[id+4],light[id+5]);
    light_pos   = readlight_pos; 
    light_color = readlight_color; 
    light_brightness = light[id+6];
}
vec3 get_light_color(vec3 normal, vec3 inter, int id){
    vec3 output = vec3(0.);
    vec3 light_pos; vec3 light_color; float light_brightness;
    get_light_attributes(id, light_pos,light_color,light_brightness);

    vec3 inter_to_light = light_pos - inter;

    float shadow = shadowCast(inter+normal*0.01,normalize(inter_to_light),length(inter_to_light));

    shadow = smoothstep(-0.1,0.8,shadow); 
    shadow = 0.0 + (shadow*1.0);
    shadow = clampn(shadow);

    vec3 lightdot = vec3(clampn(dot(-normal,normalize(inter-light_pos))));
 
    output += lightdot * light_color * light_brightness * shadow;
    output += pow(lightdot, vec3(20.)) * 0.1* shadow;
    return output;
}
void main()
{
    vec2 dim = vec2(float(windowSizeX),float(windowSizeY));
    vertexColor = vec4(1.0); 
    vec2 fragCoord = gl_FragCoord.xy/dim; 
    vec2 uv = fragCoord *2.0 - 1.; 
    uv.y *=dim.y/dim.x; 
    vec3 ro = cPos; 
    vec3 rd = vec3(uv,0.7);
    vec3 sunpos = vec3(5.,5.,-4.);
    float di = rayCast(ro,rd);
    vec3 inter = ro + rd* di;
    vec3 normal = calcNorm(inter);

    //lighting
    vec3 color = vec3(0.);
    for(float i = 0.1; i< light_count; i+=1.){
        color += get_light_color(normal, inter, int(i));
    }
//    color *= vec3(9.4,2.4,0.8);
    float dist = length(inter);
    if(di > 0.0){
        color = color; 
    }
    else{
        color = vec3(0.15,0.15,0.5);
        dist = 20.;
    }
    float y =  pow(2.718, -dist*0.038);
    y = smoothstep(0.,1.,y);
    vec3 debug = vec3(y);
    color = mix(vec3(0.2,0.2,0.4),color, 0.5+0.5*y );
    color = pow(color,vec3(0.5000));
    color = smoothstep(0.,1.,color);
    vertexColor = vec4(color,1.);
}
